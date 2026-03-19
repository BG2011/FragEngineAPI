mod models;
mod db;

use axum::{
    extract::{Path, State, FromRequestParts},
    http::{request::Parts, StatusCode},
    routing::get,
    Json, Router,
    response::{IntoResponse, Response},
};
use std::net::SocketAddr;
use dotenvy::dotenv;
use std::sync::Arc;
use crate::db::Database;
use crate::models::{Team, Player, MapStats, PlayerMapStats, H2HMatch, ApiKey};
use uuid::Uuid;

struct AppState {
    db: Database,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let db = Database::new(&database_url).await.expect("Failed to connect to database");
    let shared_state = Arc::new(AppState { db });

    // API Router
    let app = Router::new()
        .route("/", get(|| async { " FragEngine API v1.0 - Professional CS2 Data Hub" }))
        .route("/health", get(|| async { "OK" }))
        // Teams
        .route("/teams", get(get_teams))
        .route("/teams/:id", get(get_team))
        .route("/teams/:id/players", get(get_team_players))
        .route("/teams/:id/map-stats", get(get_team_map_stats))
        // Players
        .route("/players/:id/stats", get(get_player_stats))
        // Head-to-Head
        .route("/h2h/:t1/:t2", get(get_h2h_matches))
        .with_state(shared_state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");
    println!("🚀 FragEngine API listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// --- AUTH EXTRACTOR ---
pub struct AuthenticatedKey(pub ApiKey);

#[axum::async_trait]
impl FromRequestParts<Arc<AppState>> for AuthenticatedKey {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &Arc<AppState>) -> Result<Self, Self::Rejection> {
        let key = parts.headers
            .get("x-api-key")
            .or_else(|| parts.headers.get("x-rapidapi-key"))
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing API Key (x-api-key or x-rapidapi-key)").into_response())?;

        match state.db.validate_api_key(key).await {
            Ok(Some(api_key)) => {
                // Check Request Limit
                if let Some(limit) = api_key.request_limit {
                    if api_key.request_count >= limit {
                        return Err((StatusCode::TOO_MANY_REQUESTS, "Monthly API request limit exceeded. Upgrade your tier!").into_response());
                    }
                }

                // Background usage increment (non-blocking)
                let db_clone = state.db.pool.clone();
                let key_clone = key.to_string();
                tokio::spawn(async move {
                    let _ = sqlx::query("UPDATE api_keys SET request_count = request_count + 1, last_used = NOW() WHERE key = $1")
                        .bind(key_clone)
                        .execute(&db_clone)
                        .await;
                });
                Ok(AuthenticatedKey(api_key))
            },
            Ok(None) => Err((StatusCode::UNAUTHORIZED, "Invalid API Key").into_response()),
            Err(e) => {
                eprintln!("Auth Error: {}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Authentication service unavailable").into_response())
            }
        }
    }
}

// --- HANDLERS ---

async fn get_teams(
    _auth: AuthenticatedKey, 
    State(state): State<Arc<AppState>>
) -> Response {
    match state.db.get_all_teams().await {
        Ok(teams) => Json(teams).into_response(),
        Err(e) => {
            eprintln!("Error fetching teams: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching teams").into_response()
        }
    }
}

async fn get_team(
    _auth: AuthenticatedKey,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Response {
    match state.db.get_team(&id).await {
        Ok(team) => Json(team).into_response(),
        Err(e) => {
            eprintln!("Error fetching team {}: {}", id, e);
            (StatusCode::NOT_FOUND, "Team not found").into_response()
        }
    }
}

async fn get_team_players(
    AuthenticatedKey(auth): AuthenticatedKey,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Response {
    if auth.tier == "BASIC" {
        return (StatusCode::FORBIDDEN, "Tier Upgrade Required (STARTER, PRO, or ULTRA)").into_response();
    }

    let team_uuid = if let Ok(u) = Uuid::parse_str(&id) {
        Some(u)
    } else {
        match state.db.get_team_id_by_name(&id).await {
            Ok(res) => res,
            Err(_) => None,
        }
    };

    if let Some(t_uuid) = team_uuid {
        match state.db.get_team_players(t_uuid).await {
            Ok(players) => Json(players).into_response(),
            Err(_) => Json(Vec::<Player>::new()).into_response(),
        }
    } else {
        (StatusCode::NOT_FOUND, "Team not found").into_response()
    }
}

async fn get_team_map_stats(
    AuthenticatedKey(auth): AuthenticatedKey,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Response {
    if auth.tier == "BASIC" || auth.tier == "STARTER" {
        return (StatusCode::FORBIDDEN, "Tier Upgrade Required (PRO or ULTRA)").into_response();
    }

    let team_uuid = if let Ok(u) = Uuid::parse_str(&id) {
        Some(u)
    } else {
        match state.db.get_team_id_by_name(&id).await {
            Ok(res) => res,
            Err(_) => None,
        }
    };

    if let Some(t_uuid) = team_uuid {
        match state.db.get_team_map_stats(t_uuid).await {
            Ok(stats) => Json(stats).into_response(),
            Err(_) => Json(Vec::<MapStats>::new()).into_response(),
        }
    } else {
        (StatusCode::NOT_FOUND, "Team not found").into_response()
    }
}

async fn get_player_stats(
    AuthenticatedKey(auth): AuthenticatedKey,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Response {
    if auth.tier != "ULTRA" {
        return (StatusCode::FORBIDDEN, "Tier Upgrade Required (ULTRA)").into_response();
    }

    match state.db.get_player_map_stats(id).await {
        Ok(stats) => Json(stats).into_response(),
        Err(_) => Json(Vec::<PlayerMapStats>::new()).into_response(),
    }
}

async fn get_h2h_matches(
    AuthenticatedKey(auth): AuthenticatedKey,
    State(state): State<Arc<AppState>>,
    Path((t1, t2)): Path<(String, String)>,
) -> Response {
    if auth.tier == "BASIC" || auth.tier == "STARTER" {
        return (StatusCode::FORBIDDEN, "Tier Upgrade Required (PRO or ULTRA)").into_response();
    }

    let u1 = if let Ok(u) = Uuid::parse_str(&t1) {
        Some(u)
    } else {
        match state.db.get_team_id_by_name(&t1).await {
            Ok(res) => res,
            Err(_) => None,
        }
    };

    let u2 = if let Ok(u) = Uuid::parse_str(&t2) {
        Some(u)
    } else {
        match state.db.get_team_id_by_name(&t2).await {
            Ok(res) => res,
            Err(_) => None,
        }
    };

    if let (Some(uuid1), Some(uuid2)) = (u1, u2) {
        match state.db.get_h2h_matches(uuid1, uuid2).await {
            Ok(matches) => Json(matches).into_response(),
            Err(_) => Json(Vec::<H2HMatch>::new()).into_response(),
        }
    } else {
        (StatusCode::NOT_FOUND, "One or both teams not found").into_response()
    }
}

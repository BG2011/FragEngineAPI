use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct Team {
    pub id: Option<Uuid>,
    pub hltv_id: i32,
    pub name: String,
    pub url: String,
    #[serde(default)]
    #[sqlx(skip)]
    pub stats: HashMap<String, String>,
    pub avg_player_age: Option<String>,
    pub world_ranking: Option<String>,
    pub weeks_in_top30: Option<String>,
    pub valve_ranking: Option<String>,
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct MapStats {
    pub id: Option<Uuid>,
    pub team_id: Option<Uuid>,
    pub map_name: String,
    pub ct_rounds_won: Option<String>,
    pub t_rounds_won: Option<String>,
    pub round_win_after_first_kill: Option<String>,
    pub round_win_after_first_death: Option<String>,
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct Player {
    pub id: Option<Uuid>,
    pub team_id: Option<Uuid>,
    pub hltv_id: i32,
    pub name: String,
    pub status: Option<String>,
    pub time_on_team: Option<String>,
    pub maps_played: Option<i32>,
    pub rating: Option<String>,
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct PlayerMapStats {
    pub id: Option<Uuid>,
    pub player_id: Option<Uuid>,
    pub map_name: String,
    pub rating_3_0: Option<String>,
    pub t_rating: Option<String>,
    pub ct_rating: Option<String>,
    pub round_swing: Option<String>,
    pub dpr: Option<String>,
    pub kast: Option<String>,
    pub multi_kill: Option<String>,
    pub adr: Option<String>,
    pub kpr: Option<String>,
    pub total_kills: Option<String>,
    pub headshot_percentage: Option<String>,
    pub total_deaths: Option<String>,
    pub kd_ratio: Option<String>,
    pub damage_per_round: Option<String>,
    pub grenade_damage_per_round: Option<String>,
    pub maps_played: Option<String>,
    pub rounds_played: Option<String>,
    pub kills_per_round: Option<String>,
    pub assists_per_round: Option<String>,
    pub deaths_per_round: Option<String>,
    pub saved_by_teammate_per_round: Option<String>,
    pub saved_teammates_per_round: Option<String>,
    pub impact_rating: Option<String>,
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct H2HMatch {
    pub id: Option<Uuid>,
    pub team1_id: Option<Uuid>,
    pub team2_id: Option<Uuid>,
    pub team1_name: String,
    pub team2_name: String,
    pub match_date: String,
    pub event: String,
    pub map_name: String,
    pub result: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct ApiKey {
    pub id: Uuid,
    pub key: String,
    pub tier: String, // "FREE", "PRO", "ELITE"
    pub request_count: i64,
    pub request_limit: Option<i64>,
    pub last_used: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

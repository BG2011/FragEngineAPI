use sqlx::{PgPool, Row, Postgres};
use crate::models::*;
use std::error::Error;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn get_team_players(&self, team_id: Uuid) -> Result<Vec<Player>, Box<dyn Error>> {
        let rows = sqlx::query_as::<Postgres, Player>(
            "SELECT * FROM players WHERE team_id = $1 ORDER BY name ASC"
        )
        .bind(team_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn get_team_map_stats(&self, team_id: Uuid) -> Result<Vec<MapStats>, Box<dyn Error>> {
        let rows = sqlx::query_as::<Postgres, MapStats>(
            "SELECT * FROM team_map_stats WHERE team_id = $1 ORDER BY map_name ASC"
        )
        .bind(team_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn get_player_map_stats(&self, player_id: Uuid) -> Result<Vec<PlayerMapStats>, Box<dyn Error>> {
        let rows = sqlx::query_as::<Postgres, PlayerMapStats>(
            "SELECT * FROM player_map_stats WHERE player_id = $1 ORDER BY map_name ASC"
        )
        .bind(player_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn get_h2h_matches(&self, team1_id: Uuid, team2_id: Uuid) -> Result<Vec<H2HMatch>, Box<dyn Error>> {
        let rows = sqlx::query_as::<Postgres, H2HMatch>(
            r#"
            SELECT * FROM h2h_matches 
            WHERE (team1_id = $1 AND team2_id = $2) 
               OR (team1_id = $2 AND team2_id = $1)
            ORDER BY match_date DESC
            "#
        )
        .bind(team1_id)
        .bind(team2_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn get_all_teams(&self) -> Result<Vec<Team>, Box<dyn Error>> {
        let rows = sqlx::query_as::<Postgres, Team>(
            r#"
            SELECT id, hltv_id, name, url, avg_player_age, world_ranking, weeks_in_top30, valve_ranking, last_updated
            FROM teams
            ORDER BY last_updated DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn get_team(&self, identifier: &str) -> Result<Option<Team>, Box<dyn Error>> {
        if let Ok(uuid) = Uuid::parse_str(identifier) {
            sqlx::query_as::<Postgres, Team>(
                r#"
                SELECT id, hltv_id, name, url, avg_player_age, world_ranking, weeks_in_top30, valve_ranking, last_updated
                FROM teams
                WHERE id = $1
                "#
            )
            .bind(uuid)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.into())
        } else {
            sqlx::query_as::<Postgres, Team>(
                r#"
                SELECT id, hltv_id, name, url, avg_player_age, world_ranking, weeks_in_top30, valve_ranking, last_updated
                FROM teams
                WHERE name = $1
                "#
            )
            .bind(identifier)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.into())
        }
    }

    pub async fn get_team_id_by_name(&self, name: &str) -> Result<Option<Uuid>, Box<dyn Error>> {
        let row = sqlx::query("SELECT id FROM teams WHERE name = $1")
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| r.get("id")))
    }

    pub async fn validate_api_key(&self, key: &str) -> Result<Option<ApiKey>, sqlx::Error> {
        sqlx::query_as::<Postgres, ApiKey>(
            "SELECT * FROM api_keys WHERE key = $1"
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_todays_matches(&self) -> Result<Vec<TodaysMatch>, Box<dyn Error>> {
        let rows = sqlx::query_as::<Postgres, TodaysMatch>(
            "SELECT * FROM todays_matches ORDER BY last_updated DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn get_all_players(&self) -> Result<Vec<Player>, Box<dyn Error>> {
        let rows = sqlx::query_as::<Postgres, Player>(
            "SELECT * FROM players ORDER BY rating DESC NULLS LAST LIMIT 100"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }
}

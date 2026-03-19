CREATE TABLE IF NOT EXISTS teams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    hltv_id INT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    url TEXT UNIQUE NOT NULL,
    avg_player_age TEXT,
    world_ranking TEXT,
    weeks_in_top30 TEXT,
    valve_ranking TEXT,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Migration for existing teams table
ALTER TABLE teams ADD COLUMN IF NOT EXISTS last_updated TIMESTAMP WITH TIME ZONE DEFAULT NOW();

-- Coaches Table
CREATE TABLE IF NOT EXISTS coaches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID REFERENCES teams(id) ON DELETE CASCADE,
    hltv_id INT UNIQUE,
    name TEXT NOT NULL,
    time_on_team TEXT,
    maps_coached INT,
    trophies INT,
    winrate TEXT
);

-- Players Table
CREATE TABLE IF NOT EXISTS players (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID REFERENCES teams(id) ON DELETE CASCADE,
    hltv_id INT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    status TEXT,
    time_on_team TEXT,
    maps_played INT,
    rating TEXT,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Migration for existing players table
ALTER TABLE players ADD COLUMN IF NOT EXISTS last_updated TIMESTAMP WITH TIME ZONE DEFAULT NOW();

-- Team Map Stats Table
CREATE TABLE IF NOT EXISTS team_map_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID REFERENCES teams(id) ON DELETE CASCADE,
    map_name TEXT NOT NULL,
    ct_rounds_won TEXT,
    t_rounds_won TEXT,
    round_win_after_first_kill TEXT,
    round_win_after_first_death TEXT,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(team_id, map_name)
);

-- Migration for existing team_map_stats table
ALTER TABLE team_map_stats ADD COLUMN IF NOT EXISTS last_updated TIMESTAMP WITH TIME ZONE DEFAULT NOW();

-- Recent Matches Table
CREATE TABLE IF NOT EXISTS recent_matches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_map_id UUID REFERENCES team_map_stats(id) ON DELETE CASCADE,
    match_date TEXT,
    opponent TEXT,
    event TEXT,
    result TEXT
);

-- Player Map Stats Table
CREATE TABLE IF NOT EXISTS player_map_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    map_name TEXT NOT NULL,
    rating_3_0 TEXT,
    t_rating TEXT,
    ct_rating TEXT,
    round_swing TEXT,
    dpr TEXT,
    kast TEXT,
    multi_kill TEXT,
    adr TEXT,
    kpr TEXT,
    total_kills TEXT,
    headshot_percentage TEXT,
    total_deaths TEXT,
    kd_ratio TEXT,
    damage_per_round TEXT,
    grenade_damage_per_round TEXT,
    maps_played TEXT,
    rounds_played TEXT,
    kills_per_round TEXT,
    assists_per_round TEXT,
    deaths_per_round TEXT,
    saved_by_teammate_per_round TEXT,
    saved_teammates_per_round TEXT,
    impact_rating TEXT,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(player_id, map_name)
);

-- Migration for existing player_map_stats table
ALTER TABLE player_map_stats ADD COLUMN IF NOT EXISTS last_updated TIMESTAMP WITH TIME ZONE DEFAULT NOW();

-- Head-to-Head Matches Table (Historical)
CREATE TABLE IF NOT EXISTS h2h_matches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team1_id UUID REFERENCES teams(id) ON DELETE SET NULL,
    team2_id UUID REFERENCES teams(id) ON DELETE SET NULL,
    team1_name TEXT NOT NULL,
    team2_name TEXT NOT NULL,
    match_date TEXT,
    event TEXT,
    map_name TEXT,
    result TEXT,
    UNIQUE(team1_name, team2_name, match_date, map_name)
);

-- API Keys Table
CREATE TABLE IF NOT EXISTS api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key TEXT UNIQUE NOT NULL,
    tier TEXT NOT NULL CHECK (tier IN ('FREE', 'PRO', 'ELITE')),
    request_count BIGINT DEFAULT 0,
    request_limit BIGINT,
    last_used TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Optional: Initial Seed for testing
-- INSERT INTO api_keys (key, tier, request_limit) VALUES ('test_free_key', 'FREE', 1000);
-- INSERT INTO api_keys (key, tier, request_limit) VALUES ('test_pro_key', 'PRO', 10000);
-- INSERT INTO api_keys (key, tier, request_limit) VALUES ('test_elite_key', 'ELITE', NULL);

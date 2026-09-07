-- 0001_init.sql
-- Initial schema for the fantasy football webapp
-- Leagues -> Teams -> Players -> Matches -> Performances (synced from API-Football)
-- Users -> Rosters/Cards -> Transactions (in-game state)

-- ============================================================
-- Reference data (synced from API-Football)
-- ============================================================

CREATE TABLE leagues (
    id              SERIAL PRIMARY KEY,
    api_football_id INTEGER NOT NULL UNIQUE,
    name            TEXT NOT NULL,          -- 'Premier League', 'Bundesliga', ...
    country         TEXT NOT NULL,
    season          INTEGER NOT NULL,       -- 2025 for the 2025/26 season
    logo_url        TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE teams (
    id              SERIAL PRIMARY KEY,
    api_football_id INTEGER NOT NULL UNIQUE,
    league_id       INTEGER NOT NULL REFERENCES leagues(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    logo_url        TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_teams_league_id ON teams(league_id);

CREATE TABLE players (
    id              SERIAL PRIMARY KEY,
    api_football_id INTEGER NOT NULL UNIQUE,
    team_id         INTEGER REFERENCES teams(id) ON DELETE SET NULL,
    name            TEXT NOT NULL,
    position        TEXT NOT NULL,          -- 'Goalkeeper', 'Defender', 'Midfielder', 'Attacker'
    nationality     TEXT,
    age             SMALLINT,
    photo_url       TEXT,
    jersey_number   SMALLINT NOT NULL DEFAULT 0,
    total_goals     SMALLINT NOT NULL DEFAULT 0,
    total_assists   SMALLINT NOT NULL DEFAULT 0,
    pass_accuracy   NUMERIC(5,2),            
    base_price      INTEGER NOT NULL DEFAULT 0,  -- in-game currency cost to acquire, tune yourself
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_players_team_id ON players(team_id);

CREATE TABLE matches (
    id              SERIAL PRIMARY KEY,
    api_football_id INTEGER NOT NULL UNIQUE,
    league_id       INTEGER NOT NULL REFERENCES leagues(id) ON DELETE CASCADE,
    home_team_id    INTEGER NOT NULL REFERENCES teams(id),
    away_team_id    INTEGER NOT NULL REFERENCES teams(id),
    match_date      TIMESTAMPTZ NOT NULL,
    round           TEXT,                   -- 'Regular Season - 5'
    status          TEXT NOT NULL DEFAULT 'scheduled', -- scheduled | finished | postponed
    home_score      SMALLINT,
    away_score      SMALLINT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_matches_league_id ON matches(league_id);
CREATE INDEX idx_matches_date ON matches(match_date);
CREATE INDEX idx_matches_status ON matches(status);

-- ============================================================
-- Performances + points (your ingestion job writes here)
-- ============================================================

CREATE TABLE performances (
    id              SERIAL PRIMARY KEY,
    player_id       INTEGER NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    match_id        INTEGER NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    minutes_played  SMALLINT NOT NULL DEFAULT 0,
    goals           SMALLINT NOT NULL DEFAULT 0,
    assists         SMALLINT NOT NULL DEFAULT 0,
    yellow_cards    SMALLINT NOT NULL DEFAULT 0,
    red_cards       SMALLINT NOT NULL DEFAULT 0,
    saves           SMALLINT NOT NULL DEFAULT 0,       -- goalkeepers
    total_shots     SMALLINT NOT NULL DEFAULT 0,
    shots_on_target SMALLINT NOT NULL DEFAULT 0,
    pass_accuracy   NUMERIC(5,2),
    chances_created SMALLINT NOT NULL DEFAULT 0,
    total_duels     SMALLINT NOT NULL DEFAULT 0,
    duels_won       SMALLINT NOT NULL DEFAULT 0,
    clean_sheet     BOOLEAN NOT NULL DEFAULT false,
    rating          NUMERIC(3,1),                      -- API-Football's match rating, e.g. 7.4
    points_earned   INTEGER NOT NULL DEFAULT 0,        -- computed by your scoring engine
    ingested_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (player_id, match_id)
);

CREATE INDEX idx_performances_player_id ON performances(player_id);
CREATE INDEX idx_performances_match_id ON performances(match_id);

-- Tracks which fixtures have been pulled/finalized so your ingestion job
-- knows what to (re)fetch instead of guessing.
CREATE TABLE ingestion_log (
    id              SERIAL PRIMARY KEY,
    match_id        INTEGER NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    status          TEXT NOT NULL DEFAULT 'pending', -- pending | finalized | failed
    last_attempt_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (match_id)
);

-- ============================================================
-- Users + in-game economy
-- ============================================================

CREATE TABLE users (
    id              SERIAL PRIMARY KEY,
    username        TEXT NOT NULL UNIQUE,
    email           TEXT NOT NULL UNIQUE,
    password_hash   TEXT NOT NULL,
    currency_balance INTEGER NOT NULL DEFAULT 1600,  -- starting in-game money
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- A "card" is a user's owned instance of a player: it has its own level,
-- separate from the player's real-world identity. Two users can each own
-- a card for the same player, at different levels.
CREATE TABLE cards (
    id              SERIAL PRIMARY KEY,
    user_id         INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    player_id       INTEGER NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    level           SMALLINT NOT NULL DEFAULT 1,
    total_points    INTEGER NOT NULL DEFAULT 0,   -- cumulative points earned by this card
    acquired_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (user_id, player_id)
);

CREATE INDEX idx_cards_user_id ON cards(user_id);
CREATE INDEX idx_cards_player_id ON cards(player_id);

-- The active lineup: which of a user's cards are slotted into their roster,
-- and in what position. Kept separate from `cards` since a user may own
-- more cards than they can field at once.
CREATE TABLE roster_slots (
    id              SERIAL PRIMARY KEY,
    user_id         INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    card_id         INTEGER NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    slot_position   TEXT NOT NULL,   -- 'GK', 'DEF1', 'MID3', etc — define your formation slots
    UNIQUE (user_id, slot_position),
    UNIQUE (card_id)                 -- a card can only occupy one slot at a time
);

CREATE INDEX idx_roster_slots_user_id ON roster_slots(user_id);

-- Audit trail for every currency change (card upgrades, points payouts, etc).
-- Invaluable for debugging and for catching duplicate/erroneous payouts.
CREATE TABLE transactions (
    id              SERIAL PRIMARY KEY,
    user_id         INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    card_id         INTEGER REFERENCES cards(id) ON DELETE SET NULL,
    type            TEXT NOT NULL,     -- 'points_payout' | 'card_upgrade' | 'card_purchase'
    amount          INTEGER NOT NULL,  -- positive = credit, negative = debit
    description     TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_transactions_user_id ON transactions(user_id);
-- Add tournaments_won to kaiju safely
SET @exist := (SELECT count(*) FROM information_schema.columns WHERE table_schema = DATABASE() AND table_name = 'kaiju' AND column_name = 'tournaments_won');
SET @sql := IF(@exist = 0, 'ALTER TABLE kaiju ADD COLUMN tournaments_won INT NOT NULL DEFAULT 0', 'SELECT "Column already exists"');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Drop tables if they exist (to ensure fresh creation with correct collation)
SET FOREIGN_KEY_CHECKS=0;
DROP TABLE IF EXISTS tournament_matches;
DROP TABLE IF EXISTS tournament_participants;
DROP TABLE IF EXISTS tournaments;
SET FOREIGN_KEY_CHECKS=1;

-- Tournaments table
CREATE TABLE tournaments (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),
    start_time TIMESTAMP NOT NULL,
    state ENUM('Registration', 'Running', 'Finished') NOT NULL DEFAULT 'Registration',
    current_round INT NOT NULL DEFAULT 0,
    winner_kaiju_id CHAR(36),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_tournament_winner FOREIGN KEY (winner_kaiju_id) REFERENCES kaiju(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Participants (Waitlist/Bracket pool)
CREATE TABLE tournament_participants (
    tournament_id CHAR(36) NOT NULL,
    kaiju_id CHAR(36) NOT NULL,
    user_id CHAR(36) NOT NULL,
    is_bot BOOLEAN NOT NULL DEFAULT FALSE,
    joined_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tournament_id, kaiju_id),
    FOREIGN KEY (tournament_id) REFERENCES tournaments(id),
    FOREIGN KEY (kaiju_id) REFERENCES kaiju(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Matches (The Bracket)
CREATE TABLE tournament_matches (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),
    tournament_id CHAR(36) NOT NULL,
    round_number INT NOT NULL,
    match_index INT NOT NULL, -- Position in the bracket (0, 1, 2...)
    
    kaiju_a_id CHAR(36), -- Can be NULL if waiting for previous round
    kaiju_b_id CHAR(36),
    
    winner_id CHAR(36),
    
    FOREIGN KEY (tournament_id) REFERENCES tournaments(id),
    FOREIGN KEY (kaiju_a_id) REFERENCES kaiju(id),
    FOREIGN KEY (kaiju_b_id) REFERENCES kaiju(id),
    FOREIGN KEY (winner_id) REFERENCES kaiju(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

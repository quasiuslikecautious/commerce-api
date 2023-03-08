CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    session_data TEXT,
    expires_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    user_agent TEXT,
    last_activity TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    ip TEXT,
    user_id uuid,
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCES users(uuid)
);

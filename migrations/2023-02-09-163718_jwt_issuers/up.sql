CREATE TABLE IF NOT EXISTS sessions (
    id uuid PRIMARY KEY,
    session_data TEXT,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    user_agent TEXT,
    last_activity TIMESTAMP WITH TIME ZONE NOT NULL,
    ip TEXT,
    user_id uuid NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCES users(uuid)
);

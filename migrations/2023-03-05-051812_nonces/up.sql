CREATE TABLE IF NOT EXISTS nonces (
    nonce TEXT NOT NULL,
    key TEXT NOT NULL,
	created_at BIGINT NOT NULL,
    session_id TEXT PRIMARY KEY,
    CONSTRAINT fk_session
        FOREIGN KEY(session_id)
            REFERENCES sessions(id)
);

CREATE TABLE IF NOT EXISTS sessions (session_id INTEGER PRIMARY KEY AUTOINCREMENT);
CREATE TABLE IF NOT EXISTS variables (
    session_id INTEGER NOT NULL REFERENCES sessions (session_id),
    key TEXT NOT NULL,
    value REAL NOT NULL,
    PRIMARY KEY (session_id, key)
);

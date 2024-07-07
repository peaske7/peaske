-- Add migration script here
CREATE TABLE IF NOT EXISTS name (
    id SERIAL PRIMARY KEY,
    value TEXT NOT NULL
);

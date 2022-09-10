CREATE TABLE users (
  id TEXT PRIMARY KEY,
  email TEXT NOT NULL,
  hashed_password TEXT NOT NULL,
  hashed_refresh_token TEXT,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL
);

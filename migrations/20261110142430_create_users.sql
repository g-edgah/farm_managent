CREATE TYPE user_role AS ENUM ('regular', 'admin');

CREATE TABLE users (
    id            SERIAL PRIMARY KEY,
    type          user_role NOT NULL DEFAULT 'regular',
    username      TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
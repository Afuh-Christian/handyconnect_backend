-- Add migration script here

CREATE TABLE IF NOT EXISTS users (
    "id" UUID PRIMARY KEY,
    "username" TEXT NOT NULL UNIQUE
);

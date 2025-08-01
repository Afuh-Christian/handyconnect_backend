-- Add migration script here


CREATE TABLE IF NOT EXISTS users (
    "AppUserId" UUID PRIMARY KEY,
    "username" TEXT NOT NULL UNIQUE
);

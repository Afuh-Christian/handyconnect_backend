-- Add migration script here


CREATE TABLE IF NOT EXISTS users (
    "app_user_id" UUID PRIMARY KEY,
    "username" TEXT NOT NULL UNIQUE
);

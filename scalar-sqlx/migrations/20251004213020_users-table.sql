-- Add migration script here
CREATE TABLE sc__users (email TEXT PRIMARY KEY NOT NULL, name TEXT NOT NULL, password_hash TEXT NOT NULL, admin BOOL NOT NULL)

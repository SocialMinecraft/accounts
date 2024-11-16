-- Add migration script here

CREATE TABLE accounts (
    id BIGSERIAL PRIMARY KEY,
    first_name VARCHAR(50),
    birthday DATE,
    discord_id VARCHAR(100),
);
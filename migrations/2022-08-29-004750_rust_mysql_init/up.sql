-- Your SQL goes here
CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    email VARCHAR(100),
    password VARCHAR(100) NOT NULL
);
CREATE TABLE users (
    id          INTEGER PRIMARY KEY,
    email       TEXT NOT NULL UNIQUE,
    username    TEXT NOT NULL UNIQUE,
    password    TEXT NOT NULL,
    permission  INTEGER NOT NULL DEFAULT 0,
    bio         TEXT NOT NULL,
    twitter     TEXT NOT NULL,
    facebook    TEXT NOT NULL,
    tags        TEXT NOT NULL
);

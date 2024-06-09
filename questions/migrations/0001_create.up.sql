-- https://www.shuttle.rs/blog/2023/10/04/sql-in-rust

CREATE TABLE IF NOT EXISTS questions (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  content TEXT NOT NULL,
);

CREATE TABLE IF NOT EXISTS tags (
  id TEXT REFERENCES questions(id),
  tag TEXT NOT NULL
);

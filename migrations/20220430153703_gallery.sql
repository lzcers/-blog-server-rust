-- Add migration script here
CREATE TABLE IF NOT EXISTS gallery (
  id INTEGER PRIMARY KEY NOT NULL,
  url varchar(128) NOT NULL,
  datetime DATETIME,
  location varchar(128),
  description TEXT
);
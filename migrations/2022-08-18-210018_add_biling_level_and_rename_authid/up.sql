-- Your SQL goes here
ALTER TABLE users
  ADD user_tier INTEGER NOT NULL;

ALTER TABLE emailers
  DROP authentication_id;

ALTER TABLE emailers
  ADD user_id INTEGER NOT NULL default 0;
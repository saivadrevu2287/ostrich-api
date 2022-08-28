-- This file should undo anything in `up.sql`
ALTER TABLE users
  DROP user_tier;

ALTER TABLE emailers
  ADD authentication_id VARCHAR NOT NULL DEFAULT "";

ALTER TABLE emailers
  DROP user_id;
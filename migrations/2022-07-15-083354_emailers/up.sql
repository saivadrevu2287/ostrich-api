-- Your SQL goes here
CREATE TABLE emailers (
  id SERIAL PRIMARY KEY,
  search_param VARCHAR NOT NULL,
  authentication_id VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  frequency VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP,
  active BOOLEAN NOT NULL
);
-- Your SQL goes here
CREATE TABLE listing_data (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  street_address VARCHAR,
  city VARCHAR,
  state VARCHAR,
  zipcode VARCHAR,
  bedrooms INTEGER,
  bathrooms INTEGER,
  price FLOAT,
  taxes FLOAT,
  rent_estimate FLOAT,
  time_on_zillow VARCHAR,
  img_src VARCHAR,
  url VARCHAR,
  cash_on_cash FLOAT,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP,
  active BOOLEAN NOT NULL
);
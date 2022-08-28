-- This file should undo anything in `up.sql`
ALTER TABLE emailers
  DROP no_bathrooms;

ALTER TABLE emailers
  DROP notes;
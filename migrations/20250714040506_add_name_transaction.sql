-- Add migration script here
ALTER TABLE transactions
ADD COLUMN name TEXT NOT NULL DEFAULT '';

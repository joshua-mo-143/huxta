-- Add migration script here
CREATE TABLE IF NOT EXISTS requests (
	id SERIAL PRIMARY KEY,
	origin VARCHAR NOT NULL,
	version VARCHAR NOT NULL,
	headers JSONB NOT NULL,
	request_body JSONB,
	created_at timestamptz not null default current_timestamp
);

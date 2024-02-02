-- Add migration script here
CREATE type http_method AS ENUM ('Get', 'Post');
CREATE TYPE http_version AS ENUM ('Http1_0', 'Http1_1', 'Http2_0');

CREATE TABLE IF NOT EXISTS requests (
	id SERIAL PRIMARY KEY,
	method http_method NOT NULL,
	origin VARCHAR NOT NULL,
	version http_version NOT NULL,
	headers JSONB NOT NULL,
	request_body JSONB,
	created_at timestamptz not null default current_timestamp
);

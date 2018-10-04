-- Your SQL goes here
CREATE TABLE config (
  id SERIAL PRIMARY KEY,
  chat_id BIGINT NOT NULL,
  chat_name VARCHAR(255) NOT NULL,
  chat_username VARCHAR(255) NOT NULL,
  threshold SMALLINT NOT NULL DEFAULT 3,
  timeout SMALLINT NOT NULL DEFAULT 30,
  timezone SMALLINT NOT NULL DEFAULT 0,
  lang VARCHAR(8) NOT NULL DEFAULT 'en'
);
CREATE UNIQUE INDEX ON config (chat_id);

CREATE TABLE message (
  id SERIAL PRIMARY KEY,
  chat_id BIGINT NOT NULL,
  fwd_msg_id BIGINT NOT NULL,
  msg_id BIGINT NOT NULL,
  content VARCHAR(4096) NOT NULL,
  create_time TIMESTAMPTZ NOT NULL
);
CREATE INDEX ON message (chat_id);
CREATE UNIQUE INDEX ON message (chat_id, fwd_msg_id);
CREATE UNIQUE INDEX ON message (chat_id, msg_id);
CREATE INDEX ON message (chat_id, create_time);

CREATE TABLE record (
  id SERIAL PRIMARY KEY,
  chat_id BIGINT NOT NULL,
  msg_id BIGINT NOT NULL,
  msg_ids JSONB NOT NULL,
  create_time TIMESTAMPTZ NOT NULL
);
CREATE UNIQUE INDEX ON record (chat_id, msg_id);
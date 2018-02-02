-- Your SQL goes here
CREATE TABLE rockys (
  id bigint PRIMARY KEY auto_increment,
  word varchar(512) NOT NULL UNIQUE
) COLLATE utf8mb4_bin;
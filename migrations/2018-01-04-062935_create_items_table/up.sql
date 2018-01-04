-- Your SQL goes here
CREATE TABLE items (
  id bigint PRIMARY KEY auto_increment,
  title varchar(255) NOT NULL UNIQUE,
  owner varchar(255) NOT NULL,
  borrower varchar(255) NOT NULL,
  registered_date DateTime,
  due_date DateTime
) COLLATE utf8mb4_bin;
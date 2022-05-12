-- Your SQL goes here
CREATE TABLE posts (
  id int(11) NOT NULL PRIMARY KEY auto_increment,
  title VARCHAR(255) NOT NULL,
  body VARCHAR(255) NOT NULL,
  published Boolean NOT NULL DEFAULT FALSE
)
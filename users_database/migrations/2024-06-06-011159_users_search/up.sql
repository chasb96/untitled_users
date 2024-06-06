CREATE EXTENSION pg_trgm;
CREATE EXTENSION fuzzystrmatch;

CREATE TABLE users_search (
    code VARCHAR(32) NOT NULL,
    user_id INT NOT NULL,
    username VARCHAR(32) NOT NULL
);
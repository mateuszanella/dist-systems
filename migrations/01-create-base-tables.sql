CREATE DATABASE IF NOT EXISTS prod;

USE prod;

CREATE TABLE status (
    id INT
);

CREATE TABLE events (
    id INT,
    value VARCHAR(127)
);

INSERT INTO status (id) VALUES (0);
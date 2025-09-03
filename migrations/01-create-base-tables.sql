CREATE DATABASE IF NOT EXISTS prod;

USE prod;

CREATE TABLE status (
    id INT
);

CREATE TABLE event (
    id INT,
    value VARCHAR(127)
);
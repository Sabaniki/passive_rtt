-- Your SQL goes here
CREATE TABLE rtts (
    id  CHAR(64) PRIMARY KEY,
    src VARCHAR(39) NOT NULL,
    dst VARCHAR(39) NOT NULL,
    sid VARCHAR(39),
    rtt INT UNSIGNED NOT NULL
)
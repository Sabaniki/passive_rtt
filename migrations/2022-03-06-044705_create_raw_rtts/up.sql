-- Your SQL goes here
CREATE TABLE raw_rtts (
    id  SERIAL PRIMARY KEY,
    src VARCHAR(39) NOT NULL,
    dst VARCHAR(39) NOT NULL,
    sid VARCHAR(39),
    rtt INT UNSIGNED NOT NULL
)
CREATE TABLE IF NOT EXISTS configuration
(
    key        VARCHAR(255)             NOT NULL,
    value      VARCHAR(255)             NULL,
    hashed     BOOLEAN                           DEFAULT FALSE,
    created_on TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_on TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (key)
);

CREATE TABLE IF NOT EXISTS tenant
(
    id           uuid         NOT NULL,
    company_name VARCHAR(255) NOT NULL,
    email        VARCHAR(255) NOT NULL,
    phone        VARCHAR(255) NULL,
    created_on   TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_on   TIMESTAMP WITH TIME ZONE,
    UNIQUE (email),
    PRIMARY KEY (id)
);
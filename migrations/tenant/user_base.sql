CREATE TABLE IF NOT EXISTS user_base
(
    id         INTEGER GENERATED ALWAYS AS IDENTITY,
    first_name VARCHAR(255) NULL,
    last_name  VARCHAR(255) NULL,
    email      VARCHAR(255) NOT NULL,
    phone      VARCHAR(25)  NULL,
    created_on TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_on TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (email),
    UNIQUE (phone),
    UNIQUE (id)
);

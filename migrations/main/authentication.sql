CREATE TABLE IF NOT EXISTS authentication_internal_user_password
(
    id                INTEGER GENERATED ALWAYS AS IDENTITY,
    internal_user_id  INT4                     NOT NULL REFERENCES internal_user (id) ON DELETE CASCADE,
    password          VARCHAR(60)              NULL,
    previous_password VARCHAR(60)              NULL,
    created_on        TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_on        TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (id),
    UNIQUE (internal_user_id)
);

CREATE TABLE IF NOT EXISTS authentication_internal_user_reset_password_token
(
    internal_user_id INT4                     NOT NULL REFERENCES internal_user (id) ON DELETE CASCADE,
    token            uuid                              DEFAULT gen_random_uuid(),
    created_on       TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (token),
    UNIQUE (internal_user_id)
);
CREATE TABLE IF NOT EXISTS authorization_role
(
    id          INTEGER GENERATED ALWAYS AS IDENTITY,
    name        VARCHAR(100)             NOT NULL,
    description TEXT                     NULL,
    created_on  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_on  TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (name),
    UNIQUE (id)
);

CREATE TABLE IF NOT EXISTS authorization_permission
(
    id          INTEGER GENERATED ALWAYS AS IDENTITY,
    name        VARCHAR(100)             NOT NULL,
    type        VARCHAR(100)             NOT NULL,
    description TEXT                     NULL,
    created_on  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_on  TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (name, type),
    UNIQUE (id)
);

CREATE TABLE IF NOT EXISTS authorization_role_permission
(
    role_id       INT4                     NOT NULL REFERENCES authorization_role (id) ON DELETE CASCADE,
    permission_id INT4                     NOT NULL REFERENCES authorization_permission (id) ON DELETE CASCADE,
    created_on    TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_on    TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (role_id, permission_id)
);

CREATE TABLE IF NOT EXISTS authorization_role_internal_user
(
    role_id          INT4                     NOT NULL REFERENCES authorization_role (id) ON DELETE CASCADE,
    internal_user_id INT4                     NOT NULL REFERENCES internal_user (id) ON DELETE CASCADE,
    description      TEXT                     NULL,
    created_on       TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_on       TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (role_id, internal_user_id)
);


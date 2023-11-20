CREATE TABLE IF NOT EXISTS authorization_role
(
    id          INTEGER GENERATED ALWAYS AS IDENTITY,
    tenant_id   uuid                     NOT NULL REFERENCES tenant (id) ON DELETE CASCADE,
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
    tenant_id   uuid                     NOT NULL REFERENCES tenant (id) ON DELETE CASCADE,
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

CREATE TABLE IF NOT EXISTS authorization_role_admin_user
(
    role_id       INT4                     NOT NULL REFERENCES authorization_role (id) ON DELETE CASCADE,
    admin_user_id INT4                     NOT NULL REFERENCES admin_user (id) ON DELETE CASCADE,
    description   TEXT                     NULL,
    created_on    TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_on    TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (role_id, admin_user_id)
);


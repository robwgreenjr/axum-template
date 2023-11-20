CREATE TABLE IF NOT EXISTS tenant
(
    id           uuid                     DEFAULT gen_random_uuid(),
    company_name VARCHAR(255) NOT NULL,
    email        VARCHAR(255) NOT NULL,
    phone        VARCHAR(255) NULL,
    subdomain    VARCHAR(60)  NOT NULL,
    created_on   TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_on   TIMESTAMP WITH TIME ZONE,
    UNIQUE (email),
    UNIQUE (subdomain),
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS tenant_email_confirmation
(
    token      uuid                              DEFAULT gen_random_uuid(),
    tenant_id  uuid                     NOT NULL REFERENCES tenant (id) ON DELETE CASCADE,
    created_on TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (token)
);

CREATE TABLE IF NOT EXISTS tenant_database
(
    id                INTEGER GENERATED ALWAYS AS IDENTITY,
    tenant_id         uuid         NOT NULL REFERENCES tenant (id) ON DELETE CASCADE,
    url               VARCHAR(255) NOT NULL,
    username          VARCHAR(255) NOT NULL,
    password          VARCHAR(255) NOT NULL,
    minimum_idle      int4         NOT NULL    DEFAULT 2,
    maximum_pool_size int4         NOT NULL    DEFAULT 5,
    created_on        TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_on        TIMESTAMP WITH TIME ZONE,
    UNIQUE (tenant_id),
    UNIQUE (url),
    PRIMARY KEY (id)
);
TRUNCATE TABLE authorization_permission CASCADE;
TRUNCATE TABLE authorization_role CASCADE;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'internal/authorization', 'page',
        'All page access to edit authorization data.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'internal/users', 'page',
        'Allow page access to edit user data.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'internal/authorization', 'read',
        'Allow access to all authorization read endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'internal/authorization', 'write',
        'Allow access to all authorization write endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'internal/authentication', 'read',
        'Allow access to all authentication read endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'internal/authentication', 'write',
        'Allow access to all authentication write endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'configuration', 'read',
        'Allow access to all configuration read endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'configuration', 'write',
        'Allow access to all configuration write endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'internal/user', 'read',
        'Allow access to all user read endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'internal/users', 'read',
        'Allow access to all user read endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'internal/users', 'write',
        'Allow access to all user write endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'internal/user', 'write',
        'Allow access to all user write endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'tenant', 'read', 'Allow access to all tenant read endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'tenants', 'read',
        'Allow access to all tenant read endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'tenants', 'write',
        'Allow access to all tenant write endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'tenant', 'write',
        'Allow access to all tenant write endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_role
VALUES (DEFAULT, 'TOP_LEVEL', 'Has full access.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_role_permission(permission_id, role_id)
SELECT id                                                           as permission_id,
       (SELECT id FROM authorization_role WHERE name = 'TOP_LEVEL') as role_id
FROM authorization_permission
ON CONFLICT DO NOTHING;
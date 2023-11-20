TRUNCATE TABLE authorization_permission CASCADE;
TRUNCATE TABLE authorization_role CASCADE;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'admin/authorization', 'page',
        'All page access to edit authorization data.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'admin/users', 'page', 'Allow page access to edit user data.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'admin/authorization', 'read',
        'Allow access to all authorization read endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'admin/authorization', 'write',
        'Allow access to all authorization write endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'admin/authentication', 'read',
        'Allow access to all authentication read endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'admin/authentication', 'write',
        'Allow access to all authentication write endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'admin/user', 'read',
        'Allow access to all user read endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'admin/users', 'read',
        'Allow access to all user read endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'admin/users', 'write',
        'Allow access to all user write endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_permission
VALUES (DEFAULT, 'admin/user', 'write',
        'Allow access to all user write endpoints.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_role
VALUES (DEFAULT, 'TOP_LEVEL', 'Has full access.')
ON CONFLICT DO NOTHING;

INSERT INTO authorization_role_permission(permission_id, role_id)
SELECT id                                                           as permission_id,
       (SELECT id FROM authorization_role WHERE name = 'TOP_LEVEL') as role_id
FROM authorization_permission
ON CONFLICT DO NOTHING;
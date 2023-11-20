INSERT INTO user_base (first_name, last_name, email, phone)
VALUES ('User', 'Admin', 'user@admin.io', '555-555-5555')
ON CONFLICT DO NOTHING;

INSERT INTO admin_user (user_id)
VALUES ((SELECT id FROM user_base WHERE email = 'user@admin.io'))
ON CONFLICT DO NOTHING;

INSERT INTO authorization_role_admin_user (role_id, admin_user_id)
VALUES ((SELECT id FROM authorization_role WHERE name = 'TOP_LEVEL'),
        (SELECT iu.id
         FROM user_base ub
                  JOIN admin_user iu ON ub.id = iu.user_id
         WHERE ub.email = 'user@admin.io'))
ON CONFLICT DO NOTHING;

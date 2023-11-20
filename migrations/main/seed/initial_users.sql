INSERT INTO user_base (first_name, last_name, email, phone)
VALUES ('User', 'Internal', 'user@internal.io', '555-555-5555')
ON CONFLICT DO NOTHING;

INSERT INTO internal_user (user_id)
VALUES ((SELECT id FROM user_base WHERE email = 'user@internal.io'))
ON CONFLICT DO NOTHING;

INSERT INTO authorization_role_internal_user (role_id, internal_user_id)
VALUES ((SELECT id FROM authorization_role WHERE name = 'TOP_LEVEL'),
        (SELECT iu.id
         FROM user_base ub
                  JOIN internal_user iu ON ub.id = iu.user_id
         WHERE ub.email = 'user@internal.io'))
ON CONFLICT DO NOTHING;

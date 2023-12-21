--! get_auth_user_data_by_username
SELECT id, username, password_hash
FROM users
WHERE username = :username;

--! get_auth_user_data_by_id
SELECT id, username, password_hash
FROM users
WHERE id = :id;

--! get_user_permissions
SELECT DISTINCT permissions.name
FROM users
JOIN users_groups
ON users.id = users_groups.users_id
JOIN groups_permissions
ON users_groups.groups_id = groups_permissions.groups_id
JOIN permissions
ON groups_permissions.permissions_id = permissions.id
WHERE users.id = :user_id;

--! insert_a_new_admin_signup_token
INSERT INTO admin_signup_tokens (token)
VALUES (:token);

--! get_admin_token
SELECT token, used
FROM admin_signup_tokens
WHERE token = :token;

--! use_admin_token
UPDATE admin_signup_tokens
SET used = TRUE
WHERE token = :token;

--! check_if_user_exists_already
SELECT id FROM users
WHERE email = :email OR username = :username;

--! insert_new_user_settings
INSERT INTO user_settings DEFAULT VALUES returning id;

--! insert_new_user
INSERT INTO users
(user_settings_id, username, bio, email, password_hash, status)
VALUES (:user_settings_id, :username, NULL, :email, :password_hash, NULL) returning id;

--! store_user_permission
INSERT INTO users_groups (users_id, groups_id)
VALUES (
    :user_id,
    (SELECT id FROM groups WHERE name = :permission)
);

--! insert_user_image
INSERT INTO objects
(key, object_type, avatar_users_id)
VALUES (:key, 'image', :users_id);


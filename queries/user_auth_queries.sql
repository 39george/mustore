--! get_auth_user_data_by_username
SELECT id, username, password_hash
FROM users
WHERE username = :username;

--! get_auth_user_data_by_id
SELECT id, username, password_hash
FROM users
WHERE id = :id;

-----------------------------------------------------------------------------

--! check_if_user_exists_already
SELECT COUNT(*) FROM users
WHERE email = :email OR username = :username;

--! insert_new_user_settings
INSERT INTO user_settings DEFAULT VALUES returning id;

--! insert_new_user
INSERT INTO users
(user_settings_id, username, bio, avatar_url, email, password_hash, status, role, ban)
VALUES (:user_settings_id, :username, NULL, :avatar_url, :email, :password_hash, NULL, :role, NULL);


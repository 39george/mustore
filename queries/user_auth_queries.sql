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
SELECT id FROM users
WHERE email = :email OR username = :username;

--! insert_new_user_settings
INSERT INTO user_settings DEFAULT VALUES returning id;

--! insert_new_user
INSERT INTO users
(user_settings_id, username, bio, email, password_hash, status, role)
VALUES (:user_settings_id, :username, NULL, :email, :password_hash, NULL, :role) returning id;

--! insert_user_image
INSERT INTO objects
(key, object_type, avatar_users_id)
VALUES (:key, 'image', :users_id);


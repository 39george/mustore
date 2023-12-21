--! select_user_data_with_avatar_key
SELECT users.id, objects.key, username, email
FROM users
JOIN objects
ON users.id = objects.avatar_users_id
WHERE users.username = :username;

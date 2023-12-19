--! select_user_data_with_avatar_key
SELECT objects.key, username, email, role
FROM users
JOIN objects
ON users.id = objects.avatar_users_id;

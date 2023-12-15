--! get_auth_user_data_by_username
SELECT id, username, password_hash
FROM users
WHERE username = :username;

--! get_auth_user_data_by_id
SELECT id, username, password_hash
FROM users
WHERE id = :id;

--! insert_user_to_candidates
INSERT INTO user_candidates
    (username, email, password_hash, validation_token)
VALUES (:username, :email, :password_hash, :validation_token);

--! refresh_available_songs
REFRESH MATERIALIZED VIEW available_songs;

--! insert_card_token
INSERT INTO card_tokens (users_id, token)
VALUES (:user_id, :card_token);

--! fetch_card_token_by_user_id
SELECT token FROM card_tokens WHERE users_id = :user_id;

--! delete_card_token
DELETE FROM card_tokens WHERE token = :token;

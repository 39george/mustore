--! refresh_available_songs
REFRESH MATERIALIZED VIEW available_songs;

--! insert_card_token
INSERT INTO card_tokens (users_id, token)
VALUES (:user_id, :card_token);

-- SELECTING CONTENT --

--! refresh_available_songs
REFRESH MATERIALIZED VIEW available_songs;

--! fetch_card_token_by_user_id
SELECT token FROM card_tokens WHERE users_id = :user_id;

-- INSERTING CONTENT --

--! insert_card_token
INSERT INTO card_tokens (users_id, token)
VALUES (:user_id, :card_token);

--! create_service_order
INSERT INTO service_orders (offers_id, delivery_date, free_revisions_left, paid_revisions_made)
VALUES (
    :offer_id,
    CURRENT_TIMESTAMP + (
        SELECT delivery_interval FROM offers WHERE id = :offer_id
    ),
    (
        SELECT free_revisions FROM offers WHERE id = :offer_id
    ),
    0
);

-- DELETING CONTENT --

--! delete_card_token
DELETE FROM card_tokens WHERE token = :token;

-- UPDATING CONTENT --

--! update_offer_status_accepted
UPDATE offers
SET status = 'accepted', updated_at = CURRENT_TIMESTAMP
WHERE id = :offer_id;


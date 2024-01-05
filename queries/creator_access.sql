-- SELECTING CONTENT --

--! get_creator_marks_avg
SELECT AVG(mark), COUNT(mark)
FROM service_reviews
JOIN service_orders
ON service_reviews.service_orders_id = service_orders.id
JOIN services
ON service_orders.services_id = services.id
WHERE services.creator_id = :creator_id;

--! get_creator_inbox_response_rate_and_time
WITH FirstResponseTime AS (
    SELECT
        conversations.id AS conversation_id,
        MIN(messages.created_at) AS first_response_time
    FROM conversations
    JOIN participants ON conversations.id = participants.conversations_id
    JOIN messages ON conversations.id = messages.conversations_id
    WHERE participants.users_id = :user_id
        AND messages.users_id = :user_id
        AND messages.created_at > conversations.created_at
    GROUP BY conversations.id
),
ConversationResponses AS (
    SELECT
        conversations.id,
        (CASE
            WHEN frt.first_response_time IS NOT NULL AND 
                 frt.first_response_time - conversations.created_at < INTERVAL '1 day' 
            THEN 1
            ELSE 0 
         END) AS is_responded,
        frt.first_response_time - conversations.created_at AS response_time
    FROM conversations
    LEFT JOIN FirstResponseTime frt ON conversations.id = frt.conversation_id
    WHERE EXISTS (
        SELECT 1
        FROM messages
        WHERE messages.conversations_id = conversations.id
          AND messages.users_id <> :user_id
        LIMIT 1
    )
    AND conversations.created_at > NOW() - INTERVAL '1 month'
)
SELECT
    COALESCE(
        -- COUNT() will NOT count NULLS
       (COUNT(CASE WHEN is_responded = 1 THEN 1 END)::float / COUNT(*)::float) * 100,
       0
    ) AS response_rate_percentage,
    AVG(response_time)::TEXT AS average_response_time
FROM ConversationResponses;

--! get_profile_completion_value
SELECT 
    CASE
        WHEN bio IS NULL THEN 80
        ELSE 100
    END AS profile_completion_value
FROM users
WHERE users.id = :user_id;

-- UPDATING CONTENT --

-- Products

--! insert_product_and_get_product_id (description?)
INSERT INTO products(owner_id, name, description, price)
VALUES (:owher_id, :name, :description, :price) returning id;

--! insert_product_cover_object_key
INSERT INTO objects(key, object_type, cover_products_id)
VALUES (:key, 'image', :product_id);

--! insert_product_mood_by_name
INSERT INTO products_moods (products_id, moods_id)
VALUES (:product_id, (
    SELECT id FROM moods WHERE name = :mood_name
));

-- Songs

--! insert_song_and_get_song_id (secondary_genre?)
INSERT INTO songs (
    products_id,
    primary_genre,
    secondary_genre,
    sex,
    tempo,
    key,
    duration,
    lyric
)
VALUES (
    :product_id,
    (SELECT id FROM genres WHERE name = :primary_genre),
    (
        CASE
            WHEN :secondary_genre::VARCHAR(50) IS NOT NULL THEN
                (SELECT id FROM genres WHERE name = :secondary_genre)
        END
    ),
    :sex,
    :tempo,
    :key,
    :duration,
    :lyric
)
RETURNING id;

--! insert_song_master_object_key
INSERT INTO objects(key, object_type, master_songs_id)
VALUES (:key, 'audio', :song_id);

--! insert_song_master_tagged_object_key
INSERT INTO objects(key, object_type, tagged_master_songs_id)
VALUES (:key, 'audio', :song_id);

--! insert_song_multitrack_object_key
INSERT INTO objects(key, object_type, multitrack_songs_id)
VALUES (:key, 'multitrack', :song_id);

-- Offers

--! create_offer
INSERT INTO offers(conversations_id, services_id, text, price, delivery_date, free_revisions, revision_price)
VALUES (:conversations_id, :services_id, :text, :price, :delivery_date, :free_refisions, :revision_price);

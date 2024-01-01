-- SELECTING CONTENT

--! get_creator_marks_avg
SELECT AVG(mark), COUNT(mark)
FROM service_reviews
JOIN service_orders
ON service_reviews.service_orders_id = service_orders.id
JOIN services
ON service_orders.services_id = services.id
WHERE services.creator_id = :creator_id;

--! get_creator_inbox_response_rate
WITH ConversationResponses AS (
    SELECT
        conversations.id,
        BOOL_OR(participants.users_id = 1 AND messages.created_at > conversations.created_at) AS is_responded
    FROM conversations
    JOIN participants ON conversations.id = participants.conversations_id
    LEFT JOIN messages ON conversations.id = messages.conversations_id
    WHERE participants.users_id = 1
        AND (
            SELECT users_id FROM messages AS m2
            WHERE m2.conversations_id = conversations.id
            ORDER BY m2.created_at
            LIMIT 1
        ) <> 1
        AND conversations.created_at > NOW() - INTERVAL '1 month'
    GROUP BY conversations.id
)
SELECT
    CASE
        WHEN COUNT(*) = 0 THEN NULL
        ELSE (COUNT(CASE WHEN is_responded THEN 1 END)::float / COUNT(*)::float) * 100
    END AS response_rate_percentage
FROM ConversationResponses;

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


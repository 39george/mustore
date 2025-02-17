-- SELECTING CONTENT --

--! get_creator_marks_avg
SELECT AVG(mark), COUNT(mark)
FROM service_reviews
JOIN service_orders
ON service_reviews.service_orders_id = service_orders.id
JOIN offers ON service_orders.offers_id = offers.id
JOIN services ON offers.services_id = services.id
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

-- FIXME: we should return list of uncompleted values, or frontend should understand that himself.
--! get_profile_completion_value
SELECT
    CASE
        WHEN bio IS NULL THEN 80
        ELSE 100
    END AS profile_completion_value
FROM users
WHERE users.id = :user_id;

--! get_creator_songs
SELECT
    songs.id AS song_id,
    products.name,
    products.price,
    objects.key AS cover_key,
    primary_genre,
    secondary_genre,
    songs.tempo,
    songs.key AS music_key,
    songs.sex,
    songs.duration,
    songs.lyric,
    ARRAY_AGG(moods.name) AS moods,
    COUNT(likes) AS likes_count,
    COUNT(listenings) AS listenings_count
FROM products
JOIN songs ON products.id = songs.products_id
JOIN objects ON products.id = objects.cover_products_id
JOIN genres primary_genre ON songs.primary_genre = primary_genre.id
JOIN genres secondary_genre ON songs.secondary_genre = secondary_genre.id
JOIN products_moods ON products.id = products_moods.products_id
JOIN moods ON products_moods.moods_id = moods.id
JOIN likes ON songs.id = likes.songs_id
JOIN listenings ON songs.id = listenings.songs_id
WHERE products.author_id = :user_id AND products.status = :product_status
GROUP BY songs.id, products.status, products.name, products.price, objects.key, primary_genre,
        secondary_genre, songs.tempo, songs.key, songs.sex, songs.duration, songs.lyric;

-- INSERTING CONTENT --

-- Products

--! insert_product_and_get_product_id (description?)
INSERT INTO products(author_id, name, description, price)
VALUES (:owher_id, :name, :description, :price) returning id;

--! insert_product_cover_object_key
INSERT INTO objects(key, object_type, cover_products_id)
VALUES (:key, 'image', :product_id);

--! insert_product_mood_by_name
INSERT INTO products_moods (products_id, moods_id)
VALUES (:product_id, (
    SELECT id FROM moods WHERE name = :mood_name
));

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

--! insert_beat_and_get_beat_id (secondary_genre?)
INSERT INTO beats (
    products_id,
    primary_genre,
    secondary_genre,
    tempo,
    key,
    duration
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
    :tempo,
    :key,
    :duration
)
RETURNING id;

--! insert_music_product_master_object_key (song_id?, beat_id?)
INSERT INTO objects(key, object_type, master_songs_id, master_beats_id)
VALUES (:key, 'audio', :song_id, :beat_id);

--! insert_music_product_master_tagged_object_key (song_id?, beat_id?)
INSERT INTO objects(key, object_type, tagged_master_songs_id, tagged_master_beats_id)
VALUES (:key, 'audio', :song_id, :beat_id);

--! insert_music_product_multitrack_object_key (song_id?, beat_id?)
INSERT INTO objects(key, object_type, multitrack_songs_id, multitrack_beats_id)
VALUES (:key, 'multitrack', :song_id, :beat_id);

--! insert_lyric (sex?)
INSERT INTO lyrics (products_id, text, sex)
VALUES (:product_id, :text, :sex);

--! insert_cover
INSERT INTO covers (products_id)
VALUES (:product_id);

-- Services

--! insert_service_get_id (description?)
INSERT INTO services (creator_id, name, description, display_price)
VALUES (:creator_id, :name, :description, :display_price) returning id;

--! insert_mixing
INSERT INTO mixing (services_id)
VALUES (:service_id) returning id;

--! insert_song_writing
INSERT INTO song_writing (services_id)
VALUES (:service_id) returning id;

--! insert_ghost_writing (ghost_credits?)
INSERT INTO ghost_writing (services_id, ghost_credits)
VALUES (:service_id, :ghost_credits);

--! insert_beat_writing
INSERT INTO beat_writing (services_id)
VALUES (:service_id) returning id;

--! insert_cover_design
INSERT INTO cover_design (services_id)
VALUES (:service_id) returning id;

--! insert_service_cover_object_key
INSERT INTO objects(key, object_type, cover_services_id)
VALUES (:key, 'image', :service_id);

--! insert_mixing_credit_object_key
INSERT INTO objects(key, object_type, credit_mixing_id)
VALUES (:key, :object_type, :credit_mixing_id);

--! insert_song_writing_credit_object_key
INSERT INTO objects(key, object_type, credit_song_writing_id)
VALUES (:key, :object_type, :credit_song_writing_id);

--! insert_beat_writing_credit_object_key
INSERT INTO objects(key, object_type, credit_beat_writing_id)
VALUES (:key, :object_type, :credit_beat_writing_id);

--! insert_cover_design_credit_object_key
INSERT INTO objects(key, object_type, credit_cover_design_id)
VALUES (:key, :object_type, :credit_cover_design_id);

--! insert_music_service_genre (beat_writing_id?, song_writing_id?, mixing_id?)
INSERT INTO music_services_genres(genres_id, beat_writing_id, song_writing_id, mixing_id)
VALUES (
    (SELECT id FROM genres WHERE name = :genre),
    :beat_writing_id,
    :song_writing_id,
    :mixing_id
);

-- Offers

--! create_offer
INSERT INTO offers(conversations_id, services_id, creator_id, consumer_id, text, price, delivery_interval, free_revisions, revision_price)
VALUES (:conversations_id, :services_id, :creator_id, :consumer_id, :text, :price, (:delivery_interval::TEXT)::INTERVAL, :free_refisions, :revision_price);

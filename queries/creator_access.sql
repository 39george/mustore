-- SELECTING CONTENT

--! get_creator_marks_avg
SELECT AVG(mark), COUNT(mark)
FROM service_reviews
JOIN service_orders
ON service_reviews.service_orders_id = service_orders.id
JOIN services
ON service_orders.services_id = services.id
WHERE services.creator_id = :creator_id;

-- get_creator_inbox_response_rate


-- UPDATING CONTENT --

-- Products

--! insert_product_and_get_product_id
INSERT INTO products(owner_id, name, description, price)
VALUES (:owher_id, :name, :description, :price) returning id;

--! insert_product_cover_key
INSERT INTO objects(key, object_type, cover_products_id)
VALUES (:key, 'image', :product_id);

--! insert_product_tag_by_name
INSERT INTO products_tags (products_id, tags_id)
VALUES (:product_id, (
    SELECT id FROM tags WHERE name = :tag_name
));

-- Songs

--! insert_song_and_get_song_id (secondary_genre?)
INSERT INTO songs(products_id, primary_genre, secondary_genre, sex, tempo, key, duration, lyric)
VALUES (:product_id, :primary_genre, :secondary_genre, :sex, :tempo, :key, :duration, :lyric) returning id;

--! insert_song_master_key
INSERT INTO objects(key, object_type, master_songs_id)
VALUES (:key, 'audio', :song_id);

--! insert_song_multitrack_key
INSERT INTO objects(key, object_type, multitrack_songs_id)
VALUES (:key, 'multitrack', :song_id);


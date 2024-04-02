-- SELECTING CONTENT --

--: Products(product_name, author_username, price, product_cover)

--! get_liked_products : Products
SELECT
    products.name AS product_name,
    author.username AS author_username,
    products.price,
    objects.key AS product_cover
FROM products
LEFT JOIN songs ON products.id = songs.products_id
LEFT JOIN beats ON products.id = beats.products_id
LEFT JOIN lyrics ON products.id = lyrics.products_id
LEFT JOIN covers ON products.id = covers.products_id
JOIN likes ON songs.id = likes.songs_id OR beats.id = likes.beats_id OR lyrics.id = likes.lyrics_id OR covers.id = likes.covers_id AND likes.users_id = :user_id
JOIN objects ON products.id = objects.cover_products_id
JOIN users author ON products.author_id = author.id;

--! get_product_orders : Products
SELECT
    products.name AS product_name,
    author.username AS author_username,
    products.price,
    objects.key AS product_cover
FROM product_orders
JOIN users ON product_orders.consumers_id = :user_id
JOIN products ON product_orders.products_id = products.id
LEFT JOIN songs ON products.id = songs.products_id
LEFT JOIN beats ON products.id = beats.products_id
LEFT JOIN lyrics ON products.id = lyrics.products_id
LEFT JOIN covers ON products.id = covers.products_id
JOIN objects ON products.id = objects.cover_products_id
JOIN users author ON products.author_id = author.id;

-- INSERTING CONTENT --

-- accept_offer
-- INSERT INTO

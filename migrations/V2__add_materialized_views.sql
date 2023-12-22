CREATE MATERIALIZED VIEW available_songs AS (
    SELECT
        s.id AS song_id,
        p.name AS name,
        u.username AS author,
        s.tempo,
        s.key,
        s.sex,
        pg.name AS primary_genre,
        sg.name AS secondary_genre,
        p.created_at AS created_at,
        p.description,
        p.price,
        o.key AS cover_url,
        COUNT(DISTINCT l.id) AS likes, -- Make sure to count distinct records
        COUNT(DISTINCT list.id) AS listenings, -- Same for listenings
        ARRAY_AGG(DISTINCT t.name) FILTER (WHERE pt.products_id = p.id) AS vibes,
        (
            COUNT(DISTINCT l.id) * 1.0 + 
            COUNT(DISTINCT list.id) * 0.5 +
            GREATEST(100 - EXTRACT(DAY FROM CURRENT_DATE - p.created_at), 0) * 1.0
        ) AS relevance_score
    FROM songs s
    JOIN products p ON s.products_id = p.id
    JOIN genres pg ON s.primary_genre = pg.id
    JOIN users u ON p.owner_id = u.id
    LEFT JOIN genres sg ON s.secondary_genre = sg.id
    LEFT JOIN objects o ON o.cover_products_id = s.id
    LEFT JOIN likes l ON l.songs_id = s.id
    LEFT JOIN listenings list ON list.songs_id = s.id
    JOIN products_tags pt ON pt.products_id = p.id
    JOIN tags t ON pt.tags_id = t.id
    WHERE p.status = 'active'
    GROUP BY s.id, u.id, pg.name, sg.name, p.id, o.key
);

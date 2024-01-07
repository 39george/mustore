--! get_stats
(
    SELECT 'songs' AS table_name, COUNT(*) as count
    FROM songs
)
UNION ALL
(
    SELECT 'beats' AS table_name, COUNT(*) as count
    FROM beats
)
UNION ALL
(
    SELECT 'covers' AS table_name, COUNT(*) as count
    FROM covers
)
UNION ALL
(
    SELECT 'lyrics' AS table_name, COUNT(*) as count
    FROM lyrics
);

--! get_genres_list
SELECT name from genres ORDER BY name;

--! get_moods_list
SELECT name from moods ORDER BY name;

--! get_songs (sex?, tempo?, key?, genre?, mood?, user_id?) : (is_user_liked?)
SELECT 
    s.song_id,
    s.created_at,
    s.cover_url,
    s.name,
    s.author,
    s.likes,
    s.listenings,
    s.relevance_score,
    s.price,
    BOOL_OR(l.users_id = :user_id) AS is_user_liked
FROM available_songs s
LEFT JOIN likes l ON s.song_id = l.songs_id AND l.users_id = :user_id
WHERE
    (:sex::varchar(6) IS NULL OR s.sex = :sex::varchar(6))
AND ((:tempo)::smallint[] IS NULL OR (:tempo)::smallint[] IS NOT NULL
    AND s.tempo BETWEEN ((:tempo)::smallint[])[1] AND ((:tempo)::smallint[])[2])
AND ((:key)::musickey[] IS NULL OR s.key = ANY((:key)::musickey[]))
AND ((:genre)::text[] IS NULL OR s.primary_genre::text = ANY((:genre)::text[]))
AND ((:mood)::text[] IS NULL OR s.vibes::text[] && (:mood)::text[])
GROUP BY s.song_id, s.created_at, s.cover_url, s.name, s.author, s.likes, s.listenings, s.relevance_score, s.price
ORDER BY
    CASE WHEN :sort_by = 'top_wished' THEN s.likes END DESC NULLS LAST,
    CASE WHEN :sort_by = 'top_listened' THEN s.listenings END DESC NULLS LAST,
    CASE WHEN :sort_by = 'budget' THEN s.price END ASC NULLS LAST,
    CASE WHEN :sort_by = 'expensive' THEN s.price END DESC NULLS LAST,
    CASE WHEN :sort_by = 'new_first' THEN s.created_at END DESC NULLS LAST,
    CASE WHEN :sort_by = 'old_first' THEN s.created_at END ASC NULLS LAST,
    CASE WHEN :sort_by = 'relevance' THEN s.relevance_score END DESC
OFFSET :offset
LIMIT :amount;

--! get_new_songs (user_id?): (is_user_liked?)
SELECT 
s.song_id,
s.created_at,
s.cover_url,
s.name,
s.author,
s.likes,
s.price,
BOOL_OR(l.users_id = :user_id) AS is_user_liked
FROM available_songs s
LEFT JOIN likes l ON s.song_id = l.songs_id AND l.users_id = :user_id
WHERE current_timestamp - s.created_at < '2 weeks'::interval
GROUP BY s.song_id, s.created_at, s.cover_url, s.name, s.author, s.likes, s.price
ORDER BY s.created_at DESC
LIMIT :amount;

--! get_recommended_songs (user_id?) : (is_user_liked?)
SELECT 
s.song_id,
s.created_at,
s.cover_url,
s.name,
s.author,
s.likes,
s.price,
BOOL_OR(l.users_id = :user_id) AS is_user_liked
FROM available_songs s
RIGHT JOIN (
    SELECT likes.songs_id
    FROM likes
    JOIN users ON likes.users_id = users.id
    JOIN users_groups ON users.id = users_groups.users_id
    JOIN groups ON users_groups.groups_id = groups.id
    WHERE songs_id IS NOT NULL AND groups.name = 'group.administrators'
) AS admin_likes
ON song_id = admin_likes.songs_id
LEFT JOIN likes l ON s.song_id = l.songs_id AND l.users_id = :user_id
WHERE current_timestamp - s.created_at < '1 month'::interval
GROUP BY s.song_id, s.created_at, s.cover_url, s.name, s.author, s.likes, s.price
ORDER BY s.created_at DESC
LIMIT :amount;

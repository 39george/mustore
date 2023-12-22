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
SELECT name from genres;

--! get_tags_list
SELECT name from tags;

--! get_songs (sex?, tempo?, key?, genre?, tag?) : (cover_url?)
SELECT 
song_id,
created_at,
cover_url,
name,
author,
likes,
listenings,
relevance_score,
price
FROM available_songs s
WHERE
(:sex::varchar(6) IS NULL OR s.sex = :sex::varchar(6))
    AND ((:tempo)::smallint[] IS NULL OR (:tempo)::smallint[] IS NOT NULL
    AND s.tempo BETWEEN ((:tempo)::smallint[])[1] AND ((:tempo)::smallint[])[2])
AND ((:key)::musickey[] IS NULL OR s.key= ANY((:key)::musickey[]))
AND ((:genre)::text[] IS NULL OR s.primary_genre::text = ANY((:genre)::text[]))
AND ((:tag)::text[] IS NULL OR s.vibes::text[] && (:tag)::text[])
ORDER BY
    CASE WHEN :sort_by = 'top_wished' THEN likes END DESC NULLS LAST,
    CASE WHEN :sort_by = 'top_listened' THEN listenings END DESC NULLS LAST,
    CASE WHEN :sort_by = 'budget' THEN price END ASC NULLS LAST,
    CASE WHEN :sort_by = 'expensive' THEN price END DESC NULLS LAST,
    CASE WHEN :sort_by = 'new_first' THEN created_at END DESC NULLS LAST,
    CASE WHEN :sort_by = 'old_first' THEN created_at END ASC NULLS LAST,
    CASE WHEN :sort_by = 'relevance' THEN relevance_score END DESC
LIMIT :amount;

--! get_new_songs : (cover_url?)
SELECT 
song_id,
created_at,
cover_url,
name,
author,
likes,
price
FROM available_songs s
WHERE current_timestamp - created_at < '2 weeks'::interval
ORDER BY created_at DESC
LIMIT :amount;

--! get_recommended_songs
SELECT 
song_id,
created_at,
cover_url,
name,
author,
likes,
price
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
WHERE current_timestamp - created_at < '1 month'::interval
ORDER BY created_at DESC
LIMIT :amount;

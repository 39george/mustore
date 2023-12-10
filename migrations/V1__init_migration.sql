BEGIN;

-- Enums
CREATE TYPE UserRole
AS ENUM ('creator', 'consumer', 'fullstack');

CREATE TYPE MusicKey
AS ENUM
(
    'a-minor', 'a-major', 'b-flat-minor', 'b-flat-major',
    'b-minor', 'b-major', 'c-minor', 'c-major',
    'c-sharp-minor', 'c-sharp-major', 'd-minor', 'd-major',
    'e-flat-minor', 'e-flat-major', 'e-minor', 'e-major',
    'f-minor', 'f-major', 'f-sharp-minor', 'f-sharp-major',
    'g-minor', 'g-major', 'a-flat-minor', 'a-flat-major'
);

CREATE TYPE ProductStatus AS ENUM ('active', 'hidden', 'sold');

CREATE TYPE ServiceStatus AS ENUM ('active', 'hidden');

CREATE TYPE OfferStatus
AS ENUM
(
    'pending', 'accepted', 'revision', 'delivered', 'dispute',
    'fulfiled', 'rejected'
);

CREATE TYPE OrderStatus
AS ENUM ('created', 'paid', 'rejected', 'fulfiled');

-- Basic tables
CREATE TABLE genres (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name VARCHAR(50) NOT NULL
);

CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name VARCHAR(50) NOT NULL
);

CREATE TABLE superusers (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE administrators (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    username VARCHAR(30) NOT NULL,
    bio VARCHAR(400),
    avatar_url VARCHAR(200) NOT NULL,
    email VARCHAR(40) NOT NULL,
    password VARCHAR(50) NOT NULL,
    status VARCHAR(30),
    role UserRole,
    ban VARCHAR(500)
);

-- Products & tags
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    creator_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(30) NOT NULL,
    description VARCHAR(400),
    price NUMERIC(10, 2) NOT NULL,
    status ProductStatus NOT NULL,
    cover_url VARCHAR(1000)
);

CREATE TABLE products_tags (
    id SERIAL PRIMARY KEY,
    products_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    tags_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE RESTRICT
);

CREATE FUNCTION enforce_max_tags() RETURNS TRIGGER AS $$
BEGIN
    IF (SELECT COUNT(*) FROM product_tags WHERE product_id = NEW.product_id) > 3 THEN
        RAISE EXCEPTION 'Maximum of three tags per product';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER enforce_max_tags_trigger
BEFORE INSERT ON products_tags
FOR EACH ROW
EXECUTE FUNCTION enforce_max_tags();

CREATE TABLE songs (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    primary_genre INTEGER NOT NULL REFERENCES genres(id) ON DELETE RESTRICT,
    secondary_genre INTEGER REFERENCES genres(id) ON DELETE SET NULL,
    sex CHAR(1) NOT NULL CHECK (sex IN ('m', 'f')),
    tempo SMALLINT NOT NULL,
    key MusicKey NOT NULL,
    duration REAL NOT NULL,
    master_url VARCHAR(1000) NOT NULL,
    multitrack_url VARCHAR(1000) NOT NULL,
    lyric VARCHAR(1000)
);

CREATE TABLE beats (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    primary_genre INTEGER NOT NULL REFERENCES genres(id) ON DELETE RESTRICT,
    secondary_genre INTEGER REFERENCES genres(id) ON DELETE SET NULL,
    tempo SMALLINT NOT NULL,
    key MusicKey NOT NULL,
    duration REAL NOT NULL,
    master_url VARCHAR(1000) NOT NULL,
    multitrack_url VARCHAR(1000) NOT NULL
);

CREATE TABLE lyrics (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    text VARCHAR(5000) NOT NULL
);

CREATE TABLE covers (
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    id SERIAL PRIMARY KEY
);

-- Likes & listenings

CREATE TABLE likes (
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    songs_id INTEGER REFERENCES songs(id) ON DELETE CASCADE,
    beats_id INTEGER REFERENCES beats(id) ON DELETE CASCADE,
    lyrics_id INTEGER REFERENCES lyrics(id) ON DELETE CASCADE,
    covers_id INTEGER REFERENCES covers(id) ON DELETE CASCADE,
        CHECK(
        COALESCE((songs_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((beats_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((lyrics_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((covers_id)::BOOLEAN::INTEGER, 0)
        = 1
    ),
    CONSTRAINT pk_likes PRIMARY KEY (users_id, songs_id, beats_id, lyrics_id, covers_id)
);

CREATE TABLE listenings (
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    songs_id INTEGER REFERENCES songs(id) ON DELETE CASCADE,
    beats_id INTEGER REFERENCES beats(id) ON DELETE CASCADE,
    CHECK(
        COALESCE((songs_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((beats_id)::BOOLEAN::INTEGER, 0)
        = 1
    ),
    CONSTRAINT pk_listenings PRIMARY KEY (users_id, songs_id, beats_id)
);

CREATE OR REPLACE FUNCTION check_last_listening_duration() RETURNS TRIGGER AS $$
DECLARE
    last_listening_time TIMESTAMP;
    duration REAL;
BEGIN
    IF NEW.songs_id IS NOT NULL THEN
        SELECT MAX(created_at) INTO last_listening_time
        FROM listenings
        WHERE users_id = NEW.users_id AND songs_id = NEW.songs_id;
        
        SELECT duration INTO duration
        FROM songs
        WHERE id = NEW.songs_id;
    ELSEIF NEW.beats_id IS NOT NULL THEN
        SELECT MAX(created_at) INTO last_listening_time
        FROM listenings
        WHERE users_id = NEW.users_id AND beats_id = NEW.beats_id;
        
        SELECT duration INTO duration
        FROM beats
        WHERE id = NEW.beats_id;
    END IF;

    IF last_listening_time + (INTERVAL '1 second' * duration) > NEW.created_at THEN
        RAISE EXCEPTION 'Cannot insert a new listening for the same user and song/beat if the time elapsed since the last listening is less than the duration of the song/beat.';
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_last_listening_duration_trigger
BEFORE INSERT ON listenings
FOR EACH ROW
EXECUTE FUNCTION check_last_listening_duration();

-- Services
CREATE TABLE services (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    creator_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(30) NOT NULL,
    cover_url VARCHAR(1000),
    video_desc_url VARCHAR(1000),
    description VARCHAR(400),
    display_price NUMERIC(10, 2) NOT NULL,
    status ProductStatus NOT NULL DEFAULT 'active'
);

CREATE TABLE mixing (
    id SERIAL PRIMARY KEY,
    services_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    mixing_credits VARCHAR(1000)[],
    CHECK (array_length(mixing_credits, 1) < 4)
);

CREATE TABLE song_writing (
    id SERIAL PRIMARY KEY,
    services_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    song_credits VARCHAR(1000)[],
    CHECK (array_length(song_credits, 1) < 4)
);

CREATE TABLE beat_writing (
    id SERIAL PRIMARY KEY,
    services_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    beat_credits VARCHAR(1000)[],
    CHECK (array_length(beat_credits, 1) < 4)
);

CREATE TABLE ghost_writing (
    id SERIAL PRIMARY KEY,
    services_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    ghost_credits VARCHAR(5000)[],
    CHECK (array_length(ghost_credits, 1) < 6)
);

CREATE TABLE cover_design (
    id SERIAL PRIMARY KEY,
    services_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    cover_credits VARCHAR(1000)[],
    CHECK (array_length(cover_credits, 1) < 6)
);

CREATE TABLE music_services_genres (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    genres_id INTEGER NOT NULL REFERENCES genres(id) ON DELETE RESTRICT,
    beat_writing_id INTEGER REFERENCES songs(id) ON DELETE CASCADE,
    song_writing_id INTEGER REFERENCES songs(id) ON DELETE CASCADE,
    mixing_id INTEGER REFERENCES beats(id) ON DELETE CASCADE,
    CHECK (
        COALESCE((beat_writing_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((song_writing_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((mixing_id)::BOOLEAN::INTEGER, 0)
        = 1
    )
);

CREATE TABLE views (
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    services_id INTEGER REFERENCES services(id) ON DELETE CASCADE,
    songs_id INTEGER REFERENCES songs(id) ON DELETE CASCADE,
    beats_id INTEGER REFERENCES beats(id) ON DELETE CASCADE,
    lyrics_id INTEGER REFERENCES lyrics(id) ON DELETE CASCADE,
    covers_id INTEGER REFERENCES covers(id) ON DELETE CASCADE,
    CHECK(
        COALESCE((services_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((songs_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((beats_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((lyrics_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((covers_id)::BOOLEAN::INTEGER, 0)
        = 1
    ),
    CONSTRAINT pk_views PRIMARY KEY (users_id, songs_id, beats_id, lyrics_id, covers_id)
);

CREATE TABLE favorites (
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    services_id INTEGER REFERENCES services(id) ON DELETE CASCADE,
    CONSTRAINT pk_favorites PRIMARY KEY (users_id, services_id)
);

-- Orders
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    consumers_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    products_id INTEGER REFERENCES products(id) ON DELETE RESTRICT,
    services_id INTEGER REFERENCES services(id) ON DELETE RESTRICT,
    offers_id INTEGER NULL REFERENCES offers(id) ON DELETE RESTRICT,
    status OrderStatus NOT NULL DEFAULT 'created',
    CHECK (
        products_id IS NOT NULL AND services_id IS NULL and offers_id IS NULL
        OR
        products_id IS NULL AND services_id IS NOT NULL and offers_id IS NOT NULL
    )
);

CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    orders_id INTEGER NOT NULL REFERENCES orders(id) ON DELETE RESTRICT,
    description VARCHAR(200) NOT NULL,
    from_desc VARCHAR(200) NOT NULL,
    for_desc VARCHAR(200) NOT NULL,
    money_amount NUMERIC(10, 2) NOT NULL
);

CREATE TABLE reviews_data (
    id SERIAL PRIMARY KEY,
    text VARCHAR(400) NOT NULL,
    heading VARCHAR(50) NOT NULL,
    mark SMALLINT NOT NULL,
    CHECK(mark < 6 AND mark > 0)
);

CREATE TABLE reviews (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
	-- Service on which review was created
    services_id INTEGER REFERENCES services(id) ON DELETE SET NULL,
    reviews_data_id INTEGER REFERENCES reviews_data(id) ON DELETE SET NULL,
    backreview_data_id INTEGER REFERENCES reviews_data(id) ON DELETE SET NULL,
    orders_id INTEGER NOT NULL REFERENCES orders(id) ON DELETE RESTRICT
);

CREATE FUNCTION delete_row_if_null() RETURNS TRIGGER AS $$
BEGIN
    IF NEW.reviews_data_id IS NULL AND NEW.backreview_data_id IS NULL THEN
        DELETE FROM reviews WHERE id = NEW.id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_null_trigger
BEFORE INSERT OR UPDATE ON reviews
FOR EACH ROW
EXECUTE FUNCTION delete_row_if_null();

-- Messages & Conversations & Offers
CREATE TABLE conversations (
	id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    orders_id INTEGER NOT NULL REFERENCES orders(id) ON DELETE RESTRICT
);

CREATE TABLE messages (
	id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    conversations_id INTEGER NOT NULL REFERENCES conversations(id) ON DELETE RESTRICT,
    users_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
	administrators_id INTEGER REFERENCES administrators(id) ON DELETE SET NULL,
    superusers_id INTEGER REFERENCES superusers(id) ON DELETE SET NULL,
	text VARCHAR(2500) NOT NULL
);

CREATE TABLE participants (
	id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    conversations_id INTEGER NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    users_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
	administrators_id INTEGER REFERENCES administrators(id) ON DELETE CASCADE,
    superusers_id INTEGER REFERENCES superusers(id) ON DELETE CASCADE
);

CREATE TABLE offers (
	id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    conversations_id INTEGER NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    price NUMERIC(10, 2) NOT NULL,
    delivery_date TIMESTAMP NOT NULL,
    free_revisions INTEGER NOT NULL,
    revision_price NUMERIC(10, 2) NOT NULL,
    status OfferStatus NOT NULL
);

COMMIT;


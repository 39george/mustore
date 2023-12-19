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

CREATE TYPE ProductStatus AS ENUM ('moderation', 'denied', 'active', 'hidden', 'sold');

CREATE TYPE ServiceStatus AS ENUM ('moderation', 'denied', 'active', 'hidden');

CREATE TYPE OfferStatus
AS ENUM ('pending', 'accepted');

CREATE TYPE ProductOrderStatus
AS ENUM ('created', 'paid');

CREATE TYPE ServiceOrderStatus
AS ENUM ('paid', 'delivered', 'on_revision', 'dispute', 'rejected', 'fulfiled');

CREATE TYPE ObjectType
AS ENUM ('image', 'audio', 'multitrack', 'video', 'attachment');

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
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    name VARCHAR(50) NOT NULL,
    password_hash VARCHAR(500) NOT NULL
);

CREATE TABLE admin_signup_tokens(
    id SERIAL PRIMARY KEY,
    token VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    used BOOLEAN NOT NULL DEFAULT FALSE,
    admin_id INTEGER REFERENCES administrators(id)
);

CREATE TABLE user_settings (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    inbox_messages BOOL NOT NULL DEFAULT TRUE,
    order_messages BOOL NOT NULL DEFAULT TRUE,
    order_updates BOOL NOT NULL DEFAULT TRUE
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    user_settings_id INTEGER NOT NULL REFERENCES user_settings(id) ON DELETE RESTRICT,
    username VARCHAR(50) NOT NULL UNIQUE,
    bio VARCHAR(400),
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(500) NOT NULL,
    status VARCHAR(50),
    role UserRole NOT NULL,
    ban BOOL NOT NULL DEFAULT FALSE
);

-- Products & tags
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    owner_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(30) NOT NULL,
    description VARCHAR(400),
    price NUMERIC(10, 2) NOT NULL,
    status ProductStatus NOT NULL
);

-- If product is not sold and creator wants to delete it,
-- we can delete it safely.
CREATE TABLE products_tags (
    products_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    tags_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE RESTRICT,
    CONSTRAINT pk_products_tags PRIMARY KEY (products_id, tags_id)
);

CREATE TABLE songs (
    id SERIAL PRIMARY KEY,
    products_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    primary_genre INTEGER NOT NULL REFERENCES genres(id) ON DELETE RESTRICT,
    secondary_genre INTEGER REFERENCES genres(id) ON DELETE RESTRICT,
    sex CHAR(1) NOT NULL CHECK (sex IN ('m', 'f')),
    tempo SMALLINT NOT NULL,
    key MusicKey NOT NULL,
    duration REAL NOT NULL,
    lyric VARCHAR(1000) NOT NULL
);

CREATE TABLE beats (
    id SERIAL PRIMARY KEY,
    products_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    primary_genre INTEGER NOT NULL REFERENCES genres(id) ON DELETE RESTRICT,
    secondary_genre INTEGER REFERENCES genres(id) ON DELETE RESTRICT,
    tempo SMALLINT NOT NULL,
    key MusicKey NOT NULL,
    duration REAL NOT NULL
);

CREATE TABLE lyrics (
    id SERIAL PRIMARY KEY,
    products_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    text VARCHAR(5000) NOT NULL
);

CREATE TABLE covers (
    products_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
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
    description VARCHAR(400),
    display_price NUMERIC(10, 2) NOT NULL,
    status ServiceStatus NOT NULL DEFAULT 'active'
);

CREATE TABLE mixing (
    id SERIAL PRIMARY KEY,
    services_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE
);

CREATE TABLE song_writing (
    id SERIAL PRIMARY KEY,
    services_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE
);

CREATE TABLE beat_writing (
    id SERIAL PRIMARY KEY,
    services_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE
);

CREATE TABLE ghost_writing (
    id SERIAL PRIMARY KEY,
    services_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    ghost_credits VARCHAR(5000)[],
    CHECK (array_length(ghost_credits, 1) < 6)
);

CREATE TABLE cover_design (
    id SERIAL PRIMARY KEY,
    services_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE
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

CREATE TABLE favorites (
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    services_id INTEGER REFERENCES services(id) ON DELETE CASCADE,
    CONSTRAINT pk_favorites PRIMARY KEY (users_id, services_id)
);

-- Orders
CREATE TABLE product_orders (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    consumers_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    products_id INTEGER REFERENCES products(id) ON DELETE SET NULL,
    name VARCHAR(30) NOT NULL,
    price NUMERIC(10, 2) NOT NULL,
    status ProductOrderStatus NOT NULL DEFAULT 'created'
);

-- Messages & Conversations & Offers
CREATE TABLE conversations (
	id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE service_orders (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    consumers_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    services_id INTEGER REFERENCES services(id) ON DELETE SET NULL,
    conversations_id INTEGER NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    delivery_date TIMESTAMP NOT NULL,
    revisions INTEGER NOT NULL,
    revision_price NUMERIC(10, 2) NOT NULL,
    name VARCHAR(30) NOT NULL,
    price NUMERIC(10, 2) NOT NULL,
    status ServiceOrderStatus NOT NULL DEFAULT 'paid',

    -- Value should be nulled every time when delivery time changes,
    -- except creator is already failed delivery time
    delivered_on_time BOOL DEFAULT NULL
);

CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    product_orders_id INTEGER REFERENCES product_orders(id) ON DELETE RESTRICT,
    service_orders_id INTEGER REFERENCES service_orders(id) ON DELETE RESTRICT,
    description VARCHAR(200) NOT NULL,
    from_desc VARCHAR(200) NOT NULL,
    for_desc VARCHAR(200) NOT NULL,
    money_amount NUMERIC(10, 2) NOT NULL
    CHECK(
        COALESCE((product_orders_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((service_orders_id)::BOOLEAN::INTEGER, 0)
        = 1
    )
);

CREATE TABLE reviews_data (
    id SERIAL PRIMARY KEY
);

CREATE TABLE service_reviews (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    author_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    service_orders_id INTEGER NOT NULL REFERENCES service_orders(id) ON DELETE CASCADE,
    text VARCHAR(400) NOT NULL,
    mark SMALLINT NOT NULL,
    CHECK(mark < 6 AND mark > 0)
);

CREATE TABLE consumer_reviews (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    author_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    consumer_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    service_orders_id INTEGER NOT NULL REFERENCES service_orders(id) ON DELETE RESTRICT,
    text VARCHAR(400) NOT NULL,
    mark SMALLINT NOT NULL,
    CHECK(mark < 6 AND mark > 0)
);

CREATE TABLE messages (
	id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    conversations_id INTEGER NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    service_orders_id INTEGER REFERENCES service_orders(id) ON DELETE CASCADE,
    users_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
	administrators_id INTEGER REFERENCES administrators(id) ON DELETE SET NULL,
    superusers_id INTEGER REFERENCES superusers(id) ON DELETE SET NULL,
    messages_id INTEGER REFERENCES messages(id) ON DELETE SET NULL,
	text VARCHAR(2500) NOT NULL,
    CHECK(
        COALESCE((conversations_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((service_orders_id)::BOOLEAN::INTEGER, 0)
        = 1
    ),
    CHECK(
        COALESCE((users_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((administrators_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((superusers_id)::BOOLEAN::INTEGER, 0)
        = 1
    ),
    CHECK (
        (messages_id IS NULL) OR (messages_id != id)
    )
);

CREATE OR REPLACE FUNCTION check_conversations_id()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.conversations_id IS NULL THEN
        RETURN NEW;
    END IF;

    IF EXISTS (
        SELECT 1 FROM messages WHERE id = NEW.messages_id AND conversations_id = NEW.conversations_id
    ) THEN
        RETURN NEW;
    ELSE
        RAISE EXCEPTION 'Invalid conversations_id';
    END IF;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_conversations_id_trigger
BEFORE INSERT OR UPDATE ON messages
FOR EACH ROW EXECUTE FUNCTION check_conversations_id();

CREATE TABLE participants (
	id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    conversations_id INTEGER REFERENCES conversations(id) ON DELETE CASCADE,
    service_orders_id INTEGER REFERENCES service_orders(id) ON DELETE CASCADE,
    users_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
	administrators_id INTEGER REFERENCES administrators(id) ON DELETE CASCADE,
    superusers_id INTEGER REFERENCES superusers(id) ON DELETE CASCADE,
    CHECK(
        COALESCE((conversations_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((service_orders_id)::BOOLEAN::INTEGER, 0)
        = 1
    ),
    CHECK(
        COALESCE((users_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((administrators_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((superusers_id)::BOOLEAN::INTEGER, 0)
        = 1
    )
);

-- If offer is rejected, just delete it
CREATE TABLE offers (
	id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    conversations_id INTEGER NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    services_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    price NUMERIC(10, 2) NOT NULL,
    delivery_date TIMESTAMP NOT NULL,
    free_revisions INTEGER NOT NULL,
    revision_price NUMERIC(10, 2) NOT NULL,
    status OfferStatus NOT NULL
);

CREATE TABLE system_notifications (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    text VARCHAR(2500) NOT NULL
);

CREATE TABLE views (
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    services_id INTEGER REFERENCES services(id) ON DELETE CASCADE,
    songs_id INTEGER REFERENCES songs(id) ON DELETE CASCADE,
    beats_id INTEGER REFERENCES beats(id) ON DELETE CASCADE,
    lyrics_id INTEGER REFERENCES lyrics(id) ON DELETE CASCADE,
    covers_id INTEGER REFERENCES covers(id) ON DELETE CASCADE,
    messages_id INTEGER REFERENCES messages(id) ON DELETE CASCADE,
    system_notifications_id INTEGER REFERENCES system_notifications(id) ON DELETE CASCADE,
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
        +
        COALESCE((messages_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((system_notifications_id)::BOOLEAN::INTEGER, 0)
        = 1
    ),
    CONSTRAINT pk_views PRIMARY KEY (users_id, songs_id, beats_id, lyrics_id, covers_id, messages_id)
);

CREATE TABLE reports (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    messages_id INTEGER REFERENCES messages(id) ON DELETE CASCADE,
    products_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    services_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    is_open BOOL NOT NULL DEFAULT TRUE,
    CHECK(
        COALESCE((messages_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((products_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((services_id)::BOOLEAN::INTEGER, 0)
        = 1
    )
);

CREATE TABLE support_tickets (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    text VARCHAR(2500) NOT NULL,
    attachments VARCHAR(1000)[],
    is_open BOOL NOT NULL DEFAULT TRUE,
    CHECK (array_length(attachments, 1) < 4)
);

CREATE TABLE objects (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    key VARCHAR(500) NOT NULL UNIQUE,
    object_type ObjectType NOT NULL,
    -- we need to delete all objects in storage at first, so RESTRICT
    -- Images
    avatar_users_id INTEGER REFERENCES users(id) ON DELETE RESTRICT UNIQUE,
    cover_products_id INTEGER REFERENCES products(id) ON DELETE RESTRICT UNIQUE,
    cover_credits_cover_design_id INTEGER REFERENCES cover_design(id) ON DELETE RESTRICT,
    cover_services_id INTEGER REFERENCES services(id) ON DELETE CASCADE UNIQUE,

    -- Audio
    master_songs_id INTEGER REFERENCES songs(id) ON DELETE RESTRICT UNIQUE,
    multitrack_songs_id INTEGER REFERENCES songs(id) ON DELETE RESTRICT UNIQUE,
    master_beats_id INTEGER REFERENCES beats(id) ON DELETE RESTRICT UNIQUE,
    multitrack_beats_id INTEGER REFERENCES beats(id) ON DELETE RESTRICT UNIQUE,
    mixing_credits_mixing_id INTEGER REFERENCES mixing(id) ON DELETE RESTRICT,
    song_credits_songs_id INTEGER REFERENCES song_writing(id) ON DELETE RESTRICT,
    beat_credits_beat_writing_id INTEGER REFERENCES beat_writing(id) ON DELETE RESTRICT,

    -- Other
    video_description_services_id INTEGER REFERENCES services(id) ON DELETE RESTRICT UNIQUE,
    message_attachment INTEGER REFERENCES messages(id) ON DELETE RESTRICT,
    CHECK(
        COALESCE((avatar_users_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((cover_products_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((master_songs_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((multitrack_songs_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((master_beats_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((multitrack_beats_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((cover_services_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((video_description_services_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((mixing_credits_mixing_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((song_credits_songs_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((beat_credits_beat_writing_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((cover_credits_cover_design_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((message_attachment)::BOOLEAN::INTEGER, 0)
        = 1
    )
);

CREATE TABLE images (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    objects_id INTEGER NOT NULL REFERENCES objects(id) ON DELETE CASCADE,
    scale REAL, 
    offset_x REAL,
    offset_y REAL
);

CREATE OR REPLACE FUNCTION validate_avatar_users_id()
    RETURNS TRIGGER AS $$
    BEGIN
        IF NEW.objects_id IS NOT NULL THEN
            IF NOT EXISTS (
                SELECT 1
                FROM objects
                WHERE id = NEW.objects_id
                AND avatar_users_id IS NOT NULL
            ) THEN
                RAISE EXCEPTION 'Invalid avatar_users_id';
            END IF;
        END IF;
        RETURN NEW;
    END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_avatar_users_id
    BEFORE INSERT OR UPDATE ON images
    FOR EACH ROW
    EXECUTE FUNCTION validate_avatar_users_id();

CREATE OR REPLACE FUNCTION check_cover_credits_cover_design_limit()
    RETURNS TRIGGER AS $$
BEGIN
    IF NEW.cover_credits_cover_design_id IS NOT NULL THEN
        IF (SELECT COUNT(*) FROM objects WHERE cover_credits_cover_design_id = NEW.cover_credits_cover_design_id) >= 3 THEN
            RAISE EXCEPTION 'Only 3 cover_credits_cover_design_id values allowed per cover_design service';
        END IF;
    END IF;

    IF NEW.mixing_credits_mixing_id IS NOT NULL THEN
        IF (SELECT COUNT(*) FROM objects WHERE mixing_credits_mixing_id = NEW.mixing_credits_mixing_id) >= 3 THEN
            RAISE EXCEPTION 'Only 3 mixing_credits_mixing_id values allowed per mixing service';
        END IF;
    END IF;

    IF NEW.song_credits_songs_id IS NOT NULL THEN
        IF (SELECT COUNT(*) FROM objects WHERE song_credits_songs_id = NEW.song_credits_songs_id) >= 3 THEN
            RAISE EXCEPTION 'Only 3 song_credits_songs_id values allowed per song';
        END IF;
    END IF;

    IF NEW.beat_credits_beat_writing_id IS NOT NULL THEN
        IF (SELECT COUNT(*) FROM objects WHERE beat_credits_beat_writing_id = NEW.beat_credits_beat_writing_id) >= 3 THEN
            RAISE EXCEPTION 'Only 3 beat_credits_beat_writing_id values allowed per beat writing service';
        END IF;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER enforce_cover_credits_cover_design_limit
    BEFORE INSERT OR UPDATE ON objects
    FOR EACH ROW
    EXECUTE FUNCTION check_cover_credits_cover_design_limit();

-- Give a ban to a user, every time his strikes amount %3 = 0
-- If there are more than 1 ban, next bans should be delegated to the superuser
CREATE TABLE strikes (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    administrators_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    comment VARCHAR(1000) NOT NULL
);

-- I should create unban function in backend. And run it every 5 days for example.
CREATE TABLE bans (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    duration INTERVAL NOT NULL,
    users_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    administrators_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    comment VARCHAR(1000) NOT NULL
);


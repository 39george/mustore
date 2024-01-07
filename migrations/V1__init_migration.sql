-- Enums

CREATE TYPE MusicKey
AS ENUM
(
    'a_minor', 'a_major', 'b_flat_minor', 'b_flat_major',
    'b_minor', 'b_major', 'c_minor', 'c_major',
    'c_sharp_minor', 'c_sharp_major', 'd_minor', 'd_major',
    'e_flat_minor', 'e_flat_major', 'e_minor', 'e_major',
    'f_minor', 'f_major', 'f_sharp_minor', 'f_sharp_major',
    'g_minor', 'g_major', 'a_flat_minor', 'a_flat_major'
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
    name VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE moods (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name VARCHAR(50) NOT NULL UNIQUE
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
    ban BOOL NOT NULL DEFAULT FALSE
);

CREATE TABLE groups (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE permissions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

-- Create `users_groups` table for many-to-many
-- relationships between users and groups.
CREATE TABLE users_groups (
    users_id INTEGER REFERENCES users(id) ON DELETE CASCADE,    
    groups_id INTEGER REFERENCES groups(id) ON DELETE RESTRICT,
    PRIMARY KEY (users_id, groups_id)
);

-- Create `groups_permissions` table for many-to-many relationships
-- between groups and permissions.
CREATE TABLE groups_permissions (
    groups_id INTEGER REFERENCES groups(id) ON DELETE CASCADE,
    permissions_id INTEGER REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (groups_id, permissions_id)
);

-- Insert "users" and "administrators" groups.
INSERT INTO groups (name) VALUES ('group.creators');
INSERT INTO groups (name) VALUES ('group.consumers');
INSERT INTO groups (name) VALUES ('group.administrators');

-- Insert individual permissions.
INSERT INTO permissions (name) VALUES ('user');
INSERT INTO permissions (name) VALUES ('creator');
INSERT INTO permissions (name) VALUES ('consumer');
INSERT INTO permissions (name) VALUES ('administrator');

-- Insert group permissions.
INSERT INTO groups_permissions (groups_id, permissions_id)
VALUES (
    (SELECT id FROM groups WHERE name = 'group.creators'),
    (SELECT id FROM permissions WHERE name = 'user')
), (
    (SELECT id FROM groups WHERE name = 'group.creators'),
    (SELECT id FROM permissions WHERE name = 'creator')
), (
    (SELECT id FROM groups WHERE name = 'group.consumers'),
    (SELECT id FROM permissions WHERE name = 'user')
), (
    (SELECT id FROM groups WHERE name = 'group.consumers'),
    (SELECT id FROM permissions WHERE name = 'consumer')
), (
    (SELECT id FROM groups WHERE name = 'group.administrators'),
    (SELECT id FROM permissions WHERE name = 'user')
), (
    (SELECT id FROM groups WHERE name = 'group.administrators'),
    (SELECT id FROM permissions WHERE name = 'administrator')
);


CREATE TABLE admin_signup_tokens(
    id SERIAL PRIMARY KEY,
    token UUID NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    used BOOLEAN NOT NULL DEFAULT FALSE,
    users_id INTEGER REFERENCES users(id) ON DELETE SET NULL
);

-- Products
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    -- FIXME: Owner? Do we implementing owning here? Or just creator?
    owner_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(30) NOT NULL,
    description VARCHAR(400),
    price NUMERIC(10, 2) NOT NULL,
    status ProductStatus NOT NULL DEFAULT 'moderation'
);

-- If product is not sold and creator wants to delete it,
-- we can delete it safely.
CREATE TABLE products_moods (
    products_id INTEGER REFERENCES products(id) ON DELETE CASCADE,
    moods_id INTEGER REFERENCES moods(id) ON DELETE RESTRICT,
    CONSTRAINT pk_products_moods PRIMARY KEY (products_id, moods_id)
);

CREATE TABLE songs (
    id SERIAL PRIMARY KEY,
    products_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    primary_genre INTEGER NOT NULL REFERENCES genres(id) ON DELETE RESTRICT,
    secondary_genre INTEGER REFERENCES genres(id) ON DELETE RESTRICT,
    sex VARCHAR(6) NOT NULL CHECK (sex IN ('male', 'female')),
    tempo SMALLINT NOT NULL,
    key MusicKey NOT NULL,
    -- Duration is for the check_last_listening_duration() fn
    -- In seconds
    duration SMALLINT NOT NULL,
    lyric VARCHAR(1000) NOT NULL
);

CREATE TABLE beats (
    id SERIAL PRIMARY KEY,
    products_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    primary_genre INTEGER NOT NULL REFERENCES genres(id) ON DELETE RESTRICT,
    secondary_genre INTEGER REFERENCES genres(id) ON DELETE RESTRICT,
    tempo SMALLINT NOT NULL,
    key MusicKey NOT NULL,
    -- Duration is for the check_last_listening_duration() fn
    -- In seconds
    duration SMALLINT NOT NULL
);

CREATE TABLE lyrics (
    id SERIAL PRIMARY KEY,
    products_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    text VARCHAR(5000) NOT NULL
);

CREATE TABLE covers (
    id SERIAL PRIMARY KEY,
    products_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE
);

-- Likes & listenings

CREATE TABLE likes (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    songs_id INTEGER DEFAULT NULL REFERENCES songs(id) ON DELETE CASCADE,
    beats_id INTEGER DEFAULT NULL REFERENCES beats(id) ON DELETE CASCADE,
    lyrics_id INTEGER DEFAULT NULL REFERENCES lyrics(id) ON DELETE CASCADE,
    covers_id INTEGER DEFAULT NULL REFERENCES covers(id) ON DELETE CASCADE,
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
    UNIQUE (users_id, songs_id, beats_id, lyrics_id, covers_id)
);

CREATE TABLE listenings (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    songs_id INTEGER DEFAULT NULL REFERENCES songs(id) ON DELETE CASCADE,
    beats_id INTEGER DEFAULT NULL REFERENCES beats(id) ON DELETE CASCADE,
    CHECK(
        COALESCE((songs_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((beats_id)::BOOLEAN::INTEGER, 0)
        = 1
    )
);

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
    beat_writing_id INTEGER DEFAULT NULL REFERENCES songs(id) ON DELETE CASCADE,
    song_writing_id INTEGER DEFAULT NULL REFERENCES songs(id) ON DELETE CASCADE,
    mixing_id INTEGER DEFAULT NULL REFERENCES beats(id) ON DELETE CASCADE,
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
    users_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
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
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE service_orders (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    consumers_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    services_id INTEGER REFERENCES services(id) ON DELETE SET NULL,
    -- Temporary disable, because can't figure out why it is here.
    -- If all will OK, just delete that later.
    -- conversations_id INTEGER NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
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
    product_orders_id INTEGER DEFAULT NULL REFERENCES product_orders(id) ON DELETE RESTRICT,
    service_orders_id INTEGER DEFAULT NULL REFERENCES service_orders(id) ON DELETE RESTRICT,
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
    conversations_id INTEGER DEFAULT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    service_orders_id INTEGER DEFAULT NULL REFERENCES service_orders(id) ON DELETE CASCADE,
    services_id INTEGER DEFAULT NULL REFERENCES services(id) ON DELETE SET NULL,
    users_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    messages_id INTEGER REFERENCES messages(id) ON DELETE SET NULL,
	text VARCHAR(2500) NOT NULL,
    CHECK(
        COALESCE((conversations_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((service_orders_id)::BOOLEAN::INTEGER, 0)
        = 1
    ),
    CHECK (
        (messages_id IS NULL) OR (messages_id != id)
    )
);

CREATE TABLE participants (
	id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    conversations_id INTEGER DEFAULT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    service_orders_id INTEGER DEFAULT NULL REFERENCES service_orders(id) ON DELETE CASCADE,
    users_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    CHECK(
        COALESCE((conversations_id)::BOOLEAN::INTEGER, 0)
        +
        COALESCE((service_orders_id)::BOOLEAN::INTEGER, 0)
        = 1
    )
);

-- If offer is rejected, just delete it
CREATE TABLE offers (
	id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    conversations_id INTEGER NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    services_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
	text VARCHAR(2500) NOT NULL,
    price NUMERIC(10, 2) NOT NULL,
    delivery_date TIMESTAMP WITH TIME ZONE NOT NULL,
    free_revisions INTEGER NOT NULL,
    revision_price NUMERIC(10, 2) NOT NULL,
    status OfferStatus NOT NULL DEFAULT 'pending'
);

CREATE TABLE system_notifications (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    text VARCHAR(2500) NOT NULL
);

CREATE TABLE views (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    services_id INTEGER DEFAULT NULL REFERENCES services(id) ON DELETE CASCADE,
    songs_id INTEGER DEFAULT NULL REFERENCES songs(id) ON DELETE CASCADE,
    beats_id INTEGER DEFAULT NULL REFERENCES beats(id) ON DELETE CASCADE,
    lyrics_id INTEGER DEFAULT NULL REFERENCES lyrics(id) ON DELETE CASCADE,
    covers_id INTEGER DEFAULT NULL REFERENCES covers(id) ON DELETE CASCADE,
    messages_id INTEGER DEFAULT NULL REFERENCES messages(id) ON DELETE CASCADE,
    system_notifications_id INTEGER DEFAULT NULL REFERENCES system_notifications(id) ON DELETE CASCADE,
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
    UNIQUE (users_id, songs_id, beats_id, lyrics_id, covers_id, messages_id, system_notifications_id)
);

CREATE TABLE reports (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    users_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    messages_id INTEGER DEFAULT NULL REFERENCES messages(id) ON DELETE CASCADE,
    products_id INTEGER DEFAULT NULL REFERENCES products(id) ON DELETE CASCADE,
    services_id INTEGER DEFAULT NULL REFERENCES services(id) ON DELETE CASCADE,
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
    avatar_users_id INTEGER DEFAULT NULL REFERENCES users(id) ON DELETE RESTRICT UNIQUE,
    cover_products_id INTEGER DEFAULT NULL REFERENCES products(id) ON DELETE RESTRICT UNIQUE,
    cover_credits_cover_design_id INTEGER DEFAULT NULL REFERENCES cover_design(id) ON DELETE RESTRICT,
    cover_services_id INTEGER DEFAULT NULL REFERENCES services(id) ON DELETE CASCADE UNIQUE,

    -- Audio
    master_songs_id INTEGER DEFAULT NULL REFERENCES songs(id) ON DELETE RESTRICT UNIQUE,
    tagged_master_songs_id INTEGER DEFAULT NULL REFERENCES songs(id) ON DELETE RESTRICT UNIQUE,
    multitrack_songs_id INTEGER DEFAULT NULL REFERENCES songs(id) ON DELETE RESTRICT UNIQUE,
    master_beats_id INTEGER DEFAULT NULL REFERENCES beats(id) ON DELETE RESTRICT UNIQUE,
    tagged_master_beats_id INTEGER DEFAULT NULL REFERENCES beats(id) ON DELETE RESTRICT UNIQUE,
    multitrack_beats_id INTEGER DEFAULT NULL REFERENCES beats(id) ON DELETE RESTRICT UNIQUE,
    mixing_credits_mixing_id INTEGER DEFAULT NULL REFERENCES mixing(id) ON DELETE RESTRICT,
    song_credits_songs_id INTEGER DEFAULT NULL REFERENCES song_writing(id) ON DELETE RESTRICT,
    beat_credits_beat_writing_id INTEGER DEFAULT NULL REFERENCES beat_writing(id) ON DELETE RESTRICT,

    -- Other
    video_description_services_id INTEGER DEFAULT NULL REFERENCES services(id) ON DELETE RESTRICT UNIQUE,
    message_attachment INTEGER DEFAULT NULL REFERENCES messages(id) ON DELETE RESTRICT,
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

-- Functions

-- Function to prevent inserting listenings too frequently
CREATE OR REPLACE FUNCTION check_last_listening_duration()
RETURNS TRIGGER AS $$
DECLARE
    last_listening_time TIMESTAMP;
    v_duration REAL;
BEGIN
    -- Collect duration
    IF NEW.songs_id IS NOT NULL THEN
        SELECT MAX(created_at) INTO last_listening_time
        FROM listenings
        WHERE users_id = NEW.users_id AND songs_id = NEW.songs_id;
        
        SELECT songs.duration INTO v_duration
        FROM songs
        WHERE id = NEW.songs_id;
    ELSEIF NEW.beats_id IS NOT NULL THEN
        SELECT MAX(created_at) INTO last_listening_time
        FROM listenings
        WHERE users_id = NEW.users_id AND beats_id = NEW.beats_id;
        
        SELECT beats.duration INTO v_duration
        FROM beats
        WHERE id = NEW.beats_id;
    END IF;

    -- Check
    IF last_listening_time + (INTERVAL '1 second' * v_duration) > NEW.created_at THEN
        RAISE EXCEPTION 'Cannot insert a new listening for the same user and song/beat if the time elapsed since the last listening is less than the duration of the song/beat.';
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER check_last_listening_duration_trigger
BEFORE INSERT ON listenings
FOR EACH ROW
EXECUTE FUNCTION check_last_listening_duration();

-- Functions to check that for each image row, there are
-- existing row in the objects table which IS avatar object key.
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

-- Function for checking limits for credits objects 
CREATE OR REPLACE FUNCTION check_credits_limit()
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
CREATE TRIGGER enforce_credits_limit
BEFORE INSERT OR UPDATE ON objects
FOR EACH ROW
EXECUTE FUNCTION check_credits_limit();

-- Check maximum moods count for product
CREATE OR REPLACE FUNCTION check_moods_limit()
RETURNS TRIGGER AS $$
DECLARE
    mood_count INTEGER;
BEGIN
    -- Check mood count when inserting a new mood
    IF TG_OP = 'INSERT' THEN
        SELECT COUNT(*) INTO mood_count
        FROM products_moods
        WHERE products_id = NEW.products_id;

        IF mood_count >= 3 THEN
            RAISE EXCEPTION 'A product can have at most 3 moods.';
        END IF;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER trg_check_mood_limit
BEFORE INSERT OR DELETE ON products_moods
FOR EACH ROW EXECUTE FUNCTION check_moods_limit();

-- We only can reference messages in the same conversation (service order) 
-- Check that.
CREATE OR REPLACE FUNCTION check_conversations_id()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.messages_id IS NULL THEN
        RETURN NEW;
    END IF;

    IF EXISTS (
        SELECT 1 FROM messages WHERE id = NEW.messages_id AND conversations_id = NEW.conversations_id OR service_orders_id = NEW.service_orders_id
    ) THEN
        RETURN NEW;
    ELSE
        RAISE EXCEPTION 'Invalid conversations_id, or service_orders_id';
    END IF;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER check_conversations_id_trigger
BEFORE INSERT OR UPDATE ON messages
FOR EACH ROW EXECUTE FUNCTION check_conversations_id();

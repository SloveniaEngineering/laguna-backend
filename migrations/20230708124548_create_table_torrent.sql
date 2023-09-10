-- For UUIDs.
-- uuid_generate_v4()
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE SpeedLevel AS ENUM ('Lowspeed', 'Mediumspeed', 'Highspeed');
CREATE TYPE Genre AS ENUM (
    'Action',
    'Adventure',
    'Animation',
    'Biography',
    'Comedy',
    'Crime',
    'Documentary',
    'Drama',
    'Family',
    'Fantasy',
    'FilmNoir',
    'GameShow',
    'History',
    'Horror',
    'Musical',
    'Mystery',
    'News',
    'RealityTV',
    'Romance',
    'SciFi',
    'Short',
    'Sport',
    'TalkShow',
    'Thriller',
    'War',
    'Western'
);

CREATE TABLE IF NOT EXISTS "Torrent" (
    info_hash BYTEA PRIMARY KEY NOT NULL CHECK (length(info_hash) = 20 OR length(info_hash) = 40),
    raw BYTEA NOT NULL,
    announce_url VARCHAR(255),
    length BIGINT NOT NULL,
    file_name VARCHAR(100) NOT NULL,
    -- https://en.wikipedia.org/wiki/.nfo
    nfo TEXT,
    genre Genre,
    leech_count INTEGER NOT NULL CHECK (leech_count >= 0),
    seed_count INTEGER NOT NULL CHECK (seed_count >= 0),
    completed_count INTEGER NOT NULL CHECK (completed_count >= 0),
    speedlevel SpeedLevel NOT NULL,
    is_freeleech BOOLEAN NOT NULL,
    creation_date TIMESTAMP WITH TIME ZONE NOT NULL,
    created_by TEXT,
    uploaded_at TIMESTAMP WITH TIME ZONE NOT NULL,
    uploaded_by UUID NOT NULL,
    modded_at TIMESTAMP WITH TIME ZONE,
    modded_by UUID,
    FOREIGN KEY (uploaded_by) REFERENCES "User" (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (modded_by) REFERENCES "User" (id) ON DELETE SET NULL ON UPDATE CASCADE
);
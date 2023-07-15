-- For UUIDs.
-- uuid_generate_v4()
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE SpeedLevel AS ENUM ('Lowspeed', 'Mediumspeed', 'Highspeed');

CREATE TABLE IF NOT EXISTS "Torrent" (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    announce_url VARCHAR(255) NOT NULL,
    size INTEGER NOT NULL,
    title VARCHAR(100) NOT NULL,
    file_name VARCHAR(100) NOT NULL,
    -- https://en.wikipedia.org/wiki/.nfo
    nfo TEXT,
    leech_count INTEGER NOT NULL DEFAULT 0,
    seed_count INTEGER NOT NULL DEFAULT 0,
    completed_count INTEGER NOT NULL DEFAULT 0,
    speedlevel SpeedLevel NOT NULL DEFAULT 'Lowspeed',
    -- info_hash is SHA-256 hash of "info" section of torrent file (BitTorrent v2)
    info_hash TEXT UNIQUE NOT NULL,
    uploaded_at TIMESTAMP WITH TIME ZONE NOT NULL,
    uploaded_by UUID NOT NULL,
    modded_at TIMESTAMP WITH TIME ZONE,
    modded_by UUID,
    FOREIGN KEY (uploaded_by) REFERENCES "User" (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (modded_by) REFERENCES "User" (id) ON DELETE SET NULL ON UPDATE CASCADE
);
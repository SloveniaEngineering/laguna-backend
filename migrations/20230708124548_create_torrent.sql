-- For UUIDs.
-- uuid_generate_v4()
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS "Torrent" (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    name VARCHAR(100) UNIQUE NOT NULL,
    file_name VARCHAR(100) NOT NULL,
    -- https://en.wikipedia.org/wiki/.nfo
    nfo TEXT,
    -- info_hash is SHA-256 hash of "info" section of torrent file (BitTorrent v2)
    info_hash TEXT NOT NULL,
    uploaded_at TIMESTAMP WITH TIME ZONE NOT NULL,
    uploaded_by UUID NOT NULL REFERENCES "User" (id),
    modded_by UUID REFERENCES "User" (id)
);
-- For UUIDs.
-- uuid_generate_v4()
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Peer is esentially a M2M between User and Torrent.
CREATE TABLE IF NOT EXISTS "Peer" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    md5_hash VARCHAR(60),
    info_hash TEXT,
    ip TEXT,
    port INTEGER NOT NULL,
    agent TEXT,
    uploaded_bytes INTEGER,
    downloaded_bytes INTEGER,
    left_bytes INTEGER,
    behaviour Behaviour NOT NULL DEFAULT 'Lurker',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE,
    torrent_id UUID NOT NULL,
    user_id UUID NOT NULL,
    FOREIGN KEY (user_id) REFERENCES "User" (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (torrent_id) REFERENCES "Torrent" (id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- Peer is essentially a M2M between User and Torrent.
CREATE TABLE IF NOT EXISTS "Peer" (
    id BYTEA PRIMARY KEY NOT NULL CHECK (length(id) = 20),
    md5_hash VARCHAR(60),
    info_hash BYTEA NOT NULL CHECK (length(info_hash) = 40),
    ip INET,
    port INTEGER NOT NULL CHECK (port >= 0 AND port <= 65535),
    agent TEXT,
    uploaded_bytes BIGINT NOT NULL CHECK (uploaded_bytes >= 0),
    downloaded_bytes BIGINT NOT NULL CHECK (downloaded_bytes >= 0),
    left_bytes BIGINT NOT NULL CHECK (left_bytes >= 0),
    behaviour Behaviour NOT NULL DEFAULT 'Lurker',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE,
    user_id UUID NOT NULL,
    FOREIGN KEY (user_id) REFERENCES "User" (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (info_hash) REFERENCES "Torrent" (info_hash) ON DELETE CASCADE ON UPDATE CASCADE
);


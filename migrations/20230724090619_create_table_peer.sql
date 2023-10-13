-- Peer is essentially a M2M between User and Torrent.
CREATE TABLE IF NOT EXISTS "Peer"
(
    uuid             UUID PRIMARY KEY         NOT NULL DEFAULT uuid_generate_v4(),
    id               BYTEA                    NOT NULL CHECK (length(id) = 20), -- not necessarily unique
    md5_hash         VARCHAR(60),
    info_hash        BYTEA                    NOT NULL CHECK (length(info_hash) = 20 OR length(info_hash) = 32),
    ip               INET                     NOT NULL,
    port             INTEGER                  NOT NULL CHECK (port >= 0 AND port <= 65535),
    is_origin        BOOLEAN                  NOT NULL,
    agent            TEXT,
    uploaded_bytes   BIGINT                   NOT NULL CHECK (uploaded_bytes >= 0),
    downloaded_bytes BIGINT                   NOT NULL CHECK (downloaded_bytes >= 0),
    left_bytes       BIGINT                   NOT NULL CHECK (left_bytes >= 0),
    behaviour        Behaviour                NOT NULL,
    created_at       TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at       TIMESTAMP WITH TIME ZONE,
    created_by       UUID NOT NULL,
    FOREIGN KEY (info_hash) REFERENCES "Torrent" (info_hash) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (created_by) REFERENCES "User" (id) ON DELETE CASCADE ON UPDATE CASCADE
);


CREATE TABLE IF NOT EXISTS "Download"
(
    info_hash BYTEA                    NOT NULL,
    user_id   UUID                     NOT NULL,
    ts        TIMESTAMP WITH TIME ZONE NOT NULL UNIQUE,
    down_hash BYTEA                    NOT NULL UNIQUE CHECK (length(down_hash) = 32), -- sha256(info_hash, user_id, ts)
    PRIMARY KEY (info_hash, user_id, ts),
    FOREIGN KEY (info_hash) REFERENCES "Torrent" (info_hash) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES "User" (id) ON DELETE CASCADE
);
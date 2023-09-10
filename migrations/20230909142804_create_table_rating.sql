-- Stores ratings of torrents by users.
-- This is so that we prevent users from rating the same torrent multiple times.
-- It also allows better .torrent recommendations in the future.
CREATE TABLE IF NOT EXISTS "Rating" (
    user_id UUID NOT NULL,
    info_hash BYTEA NOT NULL CHECK (length(info_hash) = 20 OR length(info_hash) = 40),
    rating INTEGER NOT NULL CHECK (rating >= 0 AND rating <= 10),
    FOREIGN KEY (user_id) REFERENCES "User" (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (info_hash) REFERENCES "Torrent" (info_hash) ON DELETE CASCADE ON UPDATE CASCADE,
    PRIMARY KEY (user_id, info_hash)
);

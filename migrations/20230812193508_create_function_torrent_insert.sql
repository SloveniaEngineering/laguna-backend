CREATE OR REPLACE FUNCTION torrent_insert(
    IN BYTEA, -- info_hash
    IN VARCHAR(255), -- announce_url
    IN VARCHAR(100), -- title
    IN INTEGER, -- length
    IN VARCHAR(100), -- file_name
    IN TEXT, -- nfo
    IN TIMESTAMP WITH TIME ZONE, -- uploaded_at
    IN UUID, -- uploaded_by
    IN SpeedLevel -- speedlevel
)
RETURNS TABLE (LIKE "Torrent")
STRICT
ROWS 1
LANGUAGE SQL
AS $body$
    INSERT INTO "Torrent" (
        info_hash,
        announce_url,
        title,
        length,
        file_name,
        nfo,
        uploaded_at,
        uploaded_by,
        speedlevel
    )
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    RETURNING *;
$body$;
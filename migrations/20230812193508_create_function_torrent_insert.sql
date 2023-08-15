CREATE OR REPLACE FUNCTION torrent_insert(
    IN BYTEA, -- info_hash
    IN VARCHAR(255), -- announce_url
    IN VARCHAR(100), -- title
    IN BIGINT, -- length
    IN VARCHAR(100), -- file_name
    IN TEXT, -- nfo
    IN TIMESTAMP WITH TIME ZONE, -- creation_date
    IN TIMESTAMP WITH TIME ZONE, -- uploaded_at
    IN UUID -- uploaded_by
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
        creation_date,
        uploaded_at,
        uploaded_by
    )
    VALUES ($1, nullif($2, ''), $3, $4, $5, nullif($6, ''), $7, $8, $9)
    RETURNING *;
$body$;
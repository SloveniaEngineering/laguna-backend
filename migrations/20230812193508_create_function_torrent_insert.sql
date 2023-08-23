CREATE OR REPLACE FUNCTION torrent_insert(
    IN BYTEA, -- info_hash
    IN BYTEA, -- raw torrent file
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
        raw,
        announce_url,
        title,
        length,
        file_name,
        nfo,
        creation_date,
        uploaded_at,
        uploaded_by
    )
    VALUES ($1, $2, nullif($3, ''), $4, $5, $6, nullif($7, ''), $8, $9, $10)
    RETURNING *;
$body$;
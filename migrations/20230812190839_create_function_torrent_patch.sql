CREATE OR REPLACE FUNCTION torrent_patch(
    IN BYTEA, -- info_hash
    IN VARCHAR(100), -- title
    IN TEXT -- nfo
)
RETURNS TABLE (LIKE "Torrent")
STRICT
ROWS 1
LANGUAGE SQL
AS $body$
    UPDATE "Torrent" 
    SET title = $2, 
        nfo = nullif(trim($3), '')
    WHERE info_hash = $1
    RETURNING *; 
$body$;
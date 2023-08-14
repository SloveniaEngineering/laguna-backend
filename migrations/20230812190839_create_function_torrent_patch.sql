CREATE OR REPLACE FUNCTION torrent_patch(
    IN BYTEA, -- info_hash
    IN VARCHAR(100), -- title
    IN VARCHAR(100), -- file_name
    IN TEXT -- nfo
)
RETURNS TABLE (LIKE "Torrent")
STRICT
ROWS 1
LANGUAGE SQL
AS $body$
    UPDATE "Torrent" 
    SET title = $2, 
        file_name = $3, 
        nfo = nullif(trim($4), '')
    WHERE info_hash = $1
    RETURNING *; 
$body$;
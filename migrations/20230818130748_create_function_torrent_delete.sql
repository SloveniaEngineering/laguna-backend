CREATE OR REPLACE FUNCTION torrent_delete(
    IN BYTEA -- info_hash
)
RETURNS TABLE (LIKE "Torrent")
STRICT
ROWS 1
LANGUAGE SQL
AS $body$
    DELETE
    FROM "Torrent"
    WHERE info_hash = $1
    RETURNING *;
$body$;
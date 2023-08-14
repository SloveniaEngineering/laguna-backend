CREATE OR REPLACE FUNCTION torrent_get(
    IN BYTEA -- info_hash
)
RETURNS TABLE (LIKE "Torrent")
IMMUTABLE
STRICT
PARALLEL SAFE
ROWS 1
LANGUAGE SQL
AS $body$
    SELECT *
    FROM "Torrent"
    WHERE info_hash = $1;
$body$;
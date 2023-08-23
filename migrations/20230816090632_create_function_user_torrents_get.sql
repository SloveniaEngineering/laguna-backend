CREATE OR REPLACE FUNCTION user_torrents_get(
    IN UUID
)
RETURNS TABLE (LIKE "Torrent")
IMMUTABLE
STRICT
PARALLEL SAFE
LANGUAGE SQL
ROWS 5
AS $body$
    SELECT *
    FROM "Torrent"
    WHERE uploaded_by = $1;
$body$;
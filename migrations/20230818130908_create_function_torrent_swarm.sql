CREATE OR REPLACE FUNCTION torrent_swarm(
    IN BYTEA -- info_hash
)
RETURNS TABLE (LIKE "Peer")
STRICT
PARALLEL SAFE
LANGUAGE SQL
ROWS 15
AS $body$
    SELECT *
    FROM "Peer"
    WHERE info_hash = $1;
$body$;

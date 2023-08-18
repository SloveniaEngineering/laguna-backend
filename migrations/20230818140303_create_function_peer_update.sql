CREATE OR REPLACE FUNCTION peer_update(
    IN BYTEA, -- peer_id
    IN INET, -- ip
    IN INTEGER, -- port
    IN TEXT, -- agent
    IN BIGINT, -- uploaded
    IN BIGINT, -- downloaded
    IN BIGINT, -- left
    IN Behaviour, -- behaviour
    IN TIMESTAMP WITH TIME ZONE -- updated_at
)
RETURNS TABLE (LIKE "Peer")
STRICT
ROWS 1
LANGUAGE SQL
AS $body$
    UPDATE "Peer"
    SET
        ip = $2,
        port = $3,
        agent = $4,
        uploaded_bytes = $5,
        downloaded_bytes = $6,
        left_bytes = $7,
        behaviour = $8,
        updated_at = $9
    WHERE id = $1
    RETURNING *;
$body$;

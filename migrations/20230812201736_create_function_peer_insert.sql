-- Add migration script here
CREATE OR REPLACE FUNCTION peer_insert(
    IN BYTEA, -- peer_id
    IN BYTEA, -- info_hash
    IN INET, -- ip
    IN INTEGER, -- port
    IN TEXT, -- agent
    IN BIGINT, -- uploaded_bytes
    IN BIGINT, -- downloaded_bytes
    IN BIGINT, -- left_bytes
    IN TIMESTAMP WITH TIME ZONE, -- created_at
    IN UUID -- user_id
)
RETURNS TABLE (LIKE "Peer")
STRICT
ROWS 1
LANGUAGE SQL
AS $body$
    INSERT INTO "Peer" (
        id,
        info_hash,
        ip,
        port,
        agent,
        uploaded_bytes,
        downloaded_bytes,
        left_bytes,
        created_at,
        user_id
    )
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
    RETURNING *;
$body$;
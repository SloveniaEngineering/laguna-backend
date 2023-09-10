UPDATE "Peer" 
SET
    uploaded_bytes = $1,
    downloaded_bytes = $2,
    left_bytes = $3,
    behaviour = $4,
    updated_at = $5
WHERE id = $6
RETURNING
    id,
    md5_hash,
    info_hash,
    ip,
    port,
    is_origin,
    agent,
    uploaded_bytes,
    downloaded_bytes,
    left_bytes,
    behaviour AS "behaviour: Behaviour",
    created_at,
    updated_at
;
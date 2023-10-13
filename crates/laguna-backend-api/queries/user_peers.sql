SELECT uuid,
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
       updated_at,
       created_by
FROM "Peer"
WHERE created_by = $1;
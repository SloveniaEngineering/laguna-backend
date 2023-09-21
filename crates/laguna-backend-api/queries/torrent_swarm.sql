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
       updated_at
FROM "Peer"
WHERE info_hash = $1;
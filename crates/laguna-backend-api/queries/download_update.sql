UPDATE "Download"
  SET ts = $1,
      down_hash = $2
WHERE info_hash = $3
  AND user_id = $4
RETURNING info_hash,
          user_id,
          ts,
          down_hash;
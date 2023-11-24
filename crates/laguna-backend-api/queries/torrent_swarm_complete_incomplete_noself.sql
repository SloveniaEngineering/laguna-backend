SELECT COUNT(*) FILTER (WHERE left_bytes = 0) AS complete,
       COUNT(*) FILTER (WHERE left_bytes > 0) AS incomplete
FROM "Peer"
WHERE info_hash = $1
  AND id != $2
  AND behaviour NOT IN ('Stopped', 'Choked');
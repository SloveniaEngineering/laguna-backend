SELECT *
FROM "Rating"
WHERE user_id = $1
  AND info_hash = $2;
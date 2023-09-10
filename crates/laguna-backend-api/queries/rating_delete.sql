DELETE FROM "Rating"
WHERE 
    user_id = $1 AND info_hash = $2
RETURNING
    rating,
    user_id,
    info_hash
;
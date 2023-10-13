INSERT INTO "Rating" (rating,
                      user_id,
                      info_hash)
VALUES ($1,
        $2,
        $3) RETURNING
    rating,
    user_id,
    info_hash
;
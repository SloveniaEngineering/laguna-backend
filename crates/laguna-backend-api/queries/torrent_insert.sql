INSERT INTO "Torrent" (info_hash,
                       raw,
                       announce_url,
                       length,
                       file_name,
                       nfo,
                       genre,
                       leech_count,
                       seed_count,
                       completed_count,
                       speedlevel,
                       is_freeleech,
                       creation_date,
                       created_by,
                       uploaded_at,
                       uploaded_by,
                       modded_at,
                       modded_by)
VALUES ($1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7,
        $8,
        $9,
        $10,
        $11,
        $12,
        $13,
        $14,
        $15,
        $16,
        $17,
        $18) RETURNING
    info_hash,
    raw,
    announce_url,
    length,
    file_name,
    nfo,
    genre AS "genre: Genre",
    leech_count,
    seed_count,
    completed_count,
    speedlevel AS "speedlevel: SpeedLevel",
    is_freeleech,
    creation_date,
    created_by,
    uploaded_at,
    uploaded_by,
    modded_at,
    modded_by
;
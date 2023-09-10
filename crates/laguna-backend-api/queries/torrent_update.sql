UPDATE "Torrent"
SET
    nfo = $1,
    genre = $2
WHERE info_hash = $3
RETURNING
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
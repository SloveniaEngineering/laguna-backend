SELECT 
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
FROM "Torrent"
WHERE uploaded_by = $1;
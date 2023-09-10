SELECT
    AVG(rating)::FLOAT AS average,
    COUNT(*) AS count
FROM "Rating"
WHERE info_hash = $1;
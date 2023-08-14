CREATE OR REPLACE FUNCTION user_lookup(
    VARCHAR(30), -- username
    VARCHAR(50) -- email
)
RETURNS TABLE (LIKE "User")
IMMUTABLE
STRICT
PARALLEL SAFE
ROWS 1
LANGUAGE SQL
AS $body$
    SELECT *
    FROM "User"
    WHERE username = $1 OR email = $2;
$body$;
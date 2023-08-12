CREATE OR REPLACE FUNCTION user_insert(
    VARCHAR(30), -- username
    VARCHAR(50), -- email
    VARCHAR(100), -- password
    TEXT -- salt
)
RETURNS TABLE (LIKE "User")
STRICT
ROWS 1
LANGUAGE SQL
AS $body$
    INSERT INTO "User" (username, email, password, salt)
    VALUES ($1, $2, $3, $4)
    RETURNING *;
$body$;
CREATE OR REPLACE FUNCTION user_patch_login(
    IN UUID, -- user_id
    IN last_login TIMESTAMP WITH TIME ZONE -- last_login
)
RETURNS TABLE (LIKE "User")
STRICT
ROWS 1
LANGUAGE SQL
AS $body$
    UPDATE "User"
    SET last_login = $2
    WHERE id = $1
    RETURNING *;
$body$;

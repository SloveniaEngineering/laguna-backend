CREATE OR REPLACE FUNCTION user_patch(
    IN UUID, -- user_id
    IN BOOLEAN, -- is_history_private
    IN BOOLEAN, -- is_profile_private
    IN TEXT -- avatar_url
)
RETURNS TABLE (LIKE "User")
STRICT
ROWS 1
LANGUAGE SQL
AS $body$
    UPDATE "User"
    SET is_history_private = $2,
        is_profile_private = $3,
        avatar_url = nullif(trim($4), '')
    WHERE id = $1
    RETURNING *;
$body$;
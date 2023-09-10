DELETE FROM "User"
WHERE id = $1
RETURNING 
    id,
    username,
    email,
    password,
    first_login,
    last_login,
    avatar_url,
    salt,
    role AS "role: Role",
    hnr_count,
    behaviour AS "behaviour: Behaviour",
    is_enabled,
    is_donator,
    has_verified_email,
    is_profile_private
;
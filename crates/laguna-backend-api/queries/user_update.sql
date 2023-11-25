UPDATE "User"
SET avatar_url         = $1,
    is_profile_private = $2
WHERE id = $3 RETURNING
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
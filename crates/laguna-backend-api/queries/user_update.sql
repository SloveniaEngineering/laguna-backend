UPDATE "User"
SET username           = $1,
    avatar_url         = $2,
    is_profile_private = $3
WHERE id = $4 RETURNING
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
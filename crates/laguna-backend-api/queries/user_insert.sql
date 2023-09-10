INSERT INTO "User" (
    username,
    email,
    password,
    first_login,
    last_login,
    avatar_url,
    salt,
    role,
    hnr_count,
    behaviour,
    is_enabled,
    is_donator,
    has_verified_email,
    is_profile_private
)
VALUES (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9,
    $10,
    $11,
    $12,
    $13,
    $14
)
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
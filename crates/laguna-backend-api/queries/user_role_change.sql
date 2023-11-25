UPDATE "User"
SET role = $1
WHERE id = $2 RETURNING
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
  is_profile_private,
  email_confirm_hash,
  email_confirm_expiry
;
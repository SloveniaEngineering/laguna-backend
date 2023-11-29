-- For UUIDs.
-- uuid_generate_v4()
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE Role AS ENUM ('Normie', 'Verified', 'Mod', 'Admin');
CREATE TYPE Behaviour AS ENUM ('Lurker', 'Downloader', 'Freeleecher', 'Leech', 'Seed', 'Choked', 'Uploader', 'Stopped');

CREATE TABLE IF NOT EXISTS "User"
(
    id                 UUID PRIMARY KEY         NOT NULL DEFAULT uuid_generate_v4(),
    username           VARCHAR(30) UNIQUE       NOT NULL,
    email              VARCHAR(50) UNIQUE       NOT NULL,
    password           VARCHAR(100)             NOT NULL,
    -- AKA. date joined
    first_login        TIMESTAMP WITH TIME ZONE NOT NULL,
    last_login         TIMESTAMP WITH TIME ZONE NOT NULL,
    avatar_url         TEXT,
    salt               TEXT                     NOT NULL,
    role               Role                     NOT NULL,
    hnr_count          INTEGER                  NOT NULL CHECK (hnr_count >= 0),
    -- overall behaviour over all peers of this user
    behaviour          Behaviour                NOT NULL,
    is_enabled         BOOLEAN                  NOT NULL,
    is_donator         BOOLEAN                  NOT NULL,
    has_verified_email BOOLEAN                  NOT NULL,
    is_profile_private BOOLEAN                  NOT NULL,
    email_confirm_hash TEXT,
    email_confirm_expiry TIMESTAMP WITH TIME ZONE
);

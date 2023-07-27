-- For UUIDs.
-- uuid_generate_v4()
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE Role AS ENUM ('Normie', 'Verified', 'Mod', 'Admin');
CREATE TYPE Behaviour AS ENUM ('Lurker', 'Downloader', 'Freeleecher', 'Leech', 'Seed', 'Choked', 'Uploader');

CREATE TABLE IF NOT EXISTS "User" (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    username VARCHAR(30) UNIQUE NOT NULL,
    email VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(100) NOT NULL,
    -- AKA. date joined
    first_login TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    avatar_url TEXT,
    role Role NOT NULL DEFAULT 'Normie',
    -- overall behaviour over all peers of this user
    behaviour Behaviour NOT NULL DEFAULT 'Lurker',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    has_verified_email BOOLEAN NOT NULL DEFAULT FALSE,
    is_history_private BOOLEAN NOT NULL DEFAULT TRUE,
    is_profile_private BOOLEAN NOT NULL DEFAULT TRUE
);

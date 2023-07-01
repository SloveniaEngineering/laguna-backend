-- For UUIDs.
-- uuid_generate_v4()
-- More: 
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- For password hashing.
-- digest(pass, 'sha-256')
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TYPE Role AS ENUM ('Normie', 'Verified', 'Mod', 'Admin');

CREATE TABLE IF NOT EXISTS "User" (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    username VARCHAR(20) UNIQUE NOT NULL,
    email VARCHAR(40) UNIQUE NOT NULL,
    password TEXT NOT NULL,
    first_login TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    avatar_url TEXT,
    role Role NOT NULL DEFAULT 'Normie'
);
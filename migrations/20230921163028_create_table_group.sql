CREATE TABLE IF NOT EXISTS "Group"
(
    id          UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    name        VARCHAR(20)      NOT NULL UNIQUE,
    description VARCHAR(200)     NOT NULL,
    acronym     VARCHAR(4)       NOT NULL UNIQUE,
    leader      UUID             NOT NULL,
    FOREIGN KEY (leader) REFERENCES "User" (id) ON DELETE SET NULL ON UPDATE CASCADE
);

ALTER TABLE "User"
    ADD COLUMN "group" UUID REFERENCES "Group" (id) ON DELETE SET NULL ON UPDATE CASCADE;
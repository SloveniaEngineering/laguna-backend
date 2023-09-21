CREATE TABLE IF NOT EXISTS "GroupRole"
(
    id    UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    index INTEGER          NOT NULL,
    title VARCHAR(20)      NOT NULL,
    "group" UUID             NOT NULL,
    FOREIGN KEY ("group") REFERENCES "Group" (id) ON DELETE CASCADE ON UPDATE CASCADE
);

ALTER TABLE "User"
    ADD COLUMN group_role UUID REFERENCES "GroupRole" (id) ON DELETE SET NULL ON UPDATE CASCADE;
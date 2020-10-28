CREATE TABLE "posts" (
  "id" VARCHAR NOT NULL DEFAULT gen_random_uuid(),
  "title" character varying(128) NOT NULL,
  "body" TEXT NOT NULL,
  "published" BOOLEAN NOT NULL DEFAULT 'f',
  CONSTRAINT "post_id" PRIMARY KEY ("id")
)
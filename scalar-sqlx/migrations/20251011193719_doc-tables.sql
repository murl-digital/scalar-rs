-- Add migration script here
CREATE TABLE "sc__drafts" (
	"id"	TEXT NOT NULL,
	"doc"	TEXT NOT NULL,
	"inner"	TEXT NOT NULL,
	PRIMARY KEY("id")
);
CREATE TABLE "sc__published" (
	"id"	TEXT NOT NULL UNIQUE,
	"doc"	TEXT NOT NULL,
	"inner"	TEXT NOT NULL,
	PRIMARY KEY("id")
);
CREATE TABLE "sc__meta" (
	"id"	TEXT NOT NULL UNIQUE,
	"doc"	TEXT NOT NULL,
	"created_at"	TEXT NOT NULL,
	"modified_at"	TEXT NOT NULL,
	"published_at"	TEXT,
	PRIMARY KEY("id")
);

CREATE INDEX "doc" ON "sc__meta" (
	"doc"
);
CREATE INDEX "doc_drafts" ON "sc__drafts" (
	"doc"
);
CREATE INDEX "doc_published" ON "sc__published" (
	"doc"
);

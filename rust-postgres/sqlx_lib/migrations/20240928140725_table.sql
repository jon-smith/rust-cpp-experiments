CREATE TABLE "datatable"(
	"id" integer primary key generated by default as identity,
	"info" varchar not null,
	"data" json not null,
	"time" timestamp
);
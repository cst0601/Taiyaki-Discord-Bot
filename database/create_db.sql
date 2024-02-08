# create_db.sql
#
# Defines database schema.
# Running this script will drop the database and create a new one.

DROP DATABASE IF EXISTS Taiyaki;

CREATE DATABASE Taiyaki;

USE Taiyaki;

CREATE TABLE User (
	discord_id 	VARCHAR(18)		NOT NULL	PRIMARY KEY,
	username    VARCHAR(32)		NOT NULL,
	taiyaki_count   INTEGER		NOT NULL    DEFAULT 0,
	level 			INTEGER 	NOT NULL 	DEFAULT 1
);


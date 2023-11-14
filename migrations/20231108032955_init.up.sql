-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS users (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
	   	username VARCHAR(20) UNIQUE NOT NULL,
	  	email VARCHAR(30) UNIQUE NOT NULL,
	   	password VARCHAR(20) NOT NULL,
	   	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NULL,
    	is_admin BOOLEAN NOT NULL
    );



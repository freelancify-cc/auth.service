-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE 
    if NOT EXISTS UserRoles (
        id INT PRIMARY KEY NOT NULL,
        role_name VARCHAR(12) NOT NULL,
    );

CREATE TABLE 
    if NOT EXISTS Users (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        email VARCHAR(30) UNIQUE NOT NULL,
        password VARCHAR(20) NOT NULL,
        user_role INT NOT NULL,
        FOREIGN KEY (user_role) REFERENCES UserRoles(id),
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        upated_at TIMESTAMP NULL,
    );

CREATE TABLE 
    if NOT EXISTS UserProfiles (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        first_name VARCHAR(20) NOT NULL,
        second_name VARCHAR(20) NOT NULL,
        date_of_birth TIMESTAMP NOT NULL,
        username VARCHAR(10) NOT NULL,
        profile_picture_url VARCHAR NOT NULL,
        user UUID NOT NULL,
        FOREIGN KEY (user) REFERENCES User(id),
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        upated_at TIMESTAMP NULL,
    );

CREATE TABLE 
    if NOT EXISTS UserContactInformation (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        contact VARHCAR(10) NOT NULL,
        user_profile UUID NOT NULL,
        FOREIGN KEY (user_profile) REFERENCES UserProfile(id),
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        upated_at TIMESTAMP NULL,
    );

CREATE TABLE 
    if NOT EXISTS UserBankingInformation (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        bank VARCHAR(20) NOT NULL,
        account VARCHAR NOT NULL,
        user_profile UUID NOT NULL,
        FOREIGN KEY (user_profile) REFERENCES UserProfile(id),
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        upated_at TIMESTAMP NULL,
    );

-- Create a user
CREATE USER freelancify_dev WITH PASSWORD 'dev';

-- Create a database
CREATE DATABASE freelancify_user_service_dev;

-- Grant privileges to the user on the database
GRANT ALL PRIVILEGES ON DATABASE freelancify_user_service_dev TO freelancify_dev;


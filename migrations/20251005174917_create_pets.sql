-- Add migration script here
CREATE TABLE pets (
    id serial PRIMARY KEY,
    name varchar(100) NOT NULL,
    status varchar(50) NOT NULL,
);


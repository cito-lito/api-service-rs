-- Add up migration script here

CREATE TABLE trainers (
    id UUID PRIMARY KEY NOT NULL,
    name VARCHAR(100) UNIQUE NOT NULL,
    level smallserial NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE pokemons (
    id UUID PRIMARY KEY NOT NULL,
    name VARCHAR(100) NOT NULL,
    level smallserial NOT NULL,
    trainer_id UUID REFERENCES trainers(id),
    created_at TIMESTAMP NOT NULL
);

CREATE TABLE PTYPES (
    id UUID PRIMARY KEY NOT NULL,
    type_name VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE pokemon_types (
    pokemon_id UUID REFERENCES pokemons(id),
    type_id UUID REFERENCES PTYPES(id),
    PRIMARY KEY (pokemon_id, type_id)
);
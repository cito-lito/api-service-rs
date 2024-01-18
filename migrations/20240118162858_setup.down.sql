-- Add down migration script here

-- Drop the junction table first to remove dependencies
DROP TABLE IF EXISTS pokemon_types;

DROP TABLE IF EXISTS pokemons;
DROP TABLE IF EXISTS trainers;
DROP TABLE IF EXISTS PTYPES;
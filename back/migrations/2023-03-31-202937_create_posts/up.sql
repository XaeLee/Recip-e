-- Your SQL goes here
CREATE TABLE recipes (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    -- ingredients : list of (qtty, ingredient_id)
    ingr integer[][],
    -- tags : list of str
    tags text[]
);

CREATE TABLE ingredients (
    id SERIAL PRIMARY KEY,
    name_ VARCHAR(255) NOT NULL 
);
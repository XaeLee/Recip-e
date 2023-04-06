-- Your SQL goes here
CREATE TABLE recipes (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    -- tags : list of str
    tags TEXT[] NOT NULL
    constraint tags_not_null
      check ( array_position(tags, null) is null)
);

CREATE TABLE ingredients (
    id SERIAL PRIMARY KEY,
    name_ VARCHAR(255) NOT NULL,
    mesure VARCHAR(255)
);

CREATE TABLE jointures (
  recipe_id int,
  ingredient_id int,
  PRIMARY KEY (recipe_id, ingredient_id),
  FOREIGN KEY (recipe_id) REFERENCES recipes(id),
  FOREIGN KEY (ingredient_id) REFERENCES ingredients(id),
  quantity integer NOT NULL
);

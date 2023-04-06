use diesel::prelude::*;

#[derive(Queryable)]
pub struct Ingredient {
    pub id: i32,
    pub name_: String,
    pub mesure: String,
}

#[derive(Queryable)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub tags: Vec<String>
}

#[derive(Queryable)]
pub struct Jointure {
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub quantity: i32
}

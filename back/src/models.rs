use diesel::{prelude::*, sql_types::Array};

use crate::schema::ingredients;
use crate::schema::recipes;

#[derive(Queryable)]
pub struct PostIngredient {
    pub id: i32,
    pub name_: String
}

#[derive(Queryable)]
pub struct PostRecipe {
    pub id: i32,
    pub title: String,
    pub ingr: Array<Array<i32>>,
    pub tags: Array<String>
}
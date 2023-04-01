// @generated automatically by Diesel CLI.

diesel::table! {
    ingredients (id) {
        id -> Int4,
        name_ -> Varchar,
    }
}

diesel::table! {
    recipes (id) {
        id -> Int4,
        title -> Varchar,
        ingr -> Nullable<Array<Nullable<Int4>>>,
        tags -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    ingredients,
    recipes,
);

use postgres::{Client, NoTls};

//connect to db
let mut client = Client::connect("host=localhost user=admin", NoTls)?;

//declare groceries list && set(ingredients id)


//fn : get a random recipe

//fn : add a recipe

//fn : get ingredient from id

//fn : add ingredient to groceries list
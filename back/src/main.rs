use self::models::*;
use diesel::prelude::*;
use back::{*, schema::recipes};

fn main() {
    let connection = &mut establish_connection();
    let results = recipes::table
        .limit(4);

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
    }
}

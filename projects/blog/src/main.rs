use diesel::prelude::*;
use blog::connect;
use blog::models::Post;
use blog::schema::posts::dsl::posts;
use blog::schema::posts::published;


fn main() {
    let connection = &mut connect();
    let results:Vec<Post> = posts
        .filter(published.eq(true))
        .limit(5)
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}

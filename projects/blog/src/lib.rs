use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;

use crate::models::{NewPost, Post};
use crate::schema::posts::dsl::posts as dsl;

use diesel::RunQueryDsl;

use crate::schema::posts;

pub mod models;
pub mod schema;

pub fn connect() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to database")
}


pub fn find_by_title(conn: &mut PgConnection, title: &str) -> Result<Vec<Post>, diesel::result::Error> {
    dsl.select(Post::as_select())
        .filter(posts::title.eq(title))
        .load::<Post>(conn)
}
pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    let new_post = NewPost { title, body };
    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn).expect("error saving ")
}

pub fn update_post(conn: &mut PgConnection, post: &Post) -> Result<(), diesel::result::Error> {
    diesel::update(dsl.find(post.id))
        .set(posts::title.eq("Updated"))
        .returning(Post::as_returning())
        .get_result(conn)?;
    Ok(())
}

pub fn delete_post(conn: &mut PgConnection, id: i32) -> Result<(), diesel::result::Error> {
    diesel::delete(dsl.find(id))
        .execute(conn)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() -> Result<(), diesel::result::Error> {
        let mut conn = connect();
        let post = create_post(&mut conn, "title", "post");
        println!("{:?}", post);

        update_post(&mut conn, &post)?;
        delete_post(&mut conn, 1)?;
        let posts = find_by_title(&mut conn, "Updated")?;
        println!("{:#?}", posts);
        Ok(())
    }
}

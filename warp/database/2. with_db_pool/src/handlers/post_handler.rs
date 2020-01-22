use warp;

use crate::models::post::{
    PostList,
    Post,
    NewPost,
};

use dotenv::dotenv;
use std::env;

// https://github.com/sfackler/r2d2https://github.com/sfackler/r2d2
// https://docs.diesel.rs/diesel/r2d2/struct.ConnectionManager.html
// https://docs.diesel.rs/diesel/pg/struct.PgConnection.html
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

// https://docs.rs/lazy_static/1.4.0/lazy_static/
// https://www.google.com/search?&q=why+use+lazy_static+rust
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
lazy_static! {
    static ref POOL: Pool = {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set.");

        // create db connection pool
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        pool
    };
}

// Use this to debug and verify API chaining work or not.
// pub async fn repeat(input: String) -> Result<impl warp::Reply, warp::Rejection> {
//     println!("{:#?}", &input);
//     Ok(warp::reply::html(input))
// }

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = PostList::list(&conn);
    println!("{:#?}", &response);

    Ok(warp::reply::json(&response))
}

pub async fn get(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Post::find(&id, &conn);

    let reply = match response {
        Ok(post) => {
            println!("{:#?}", &post);
            post
        },
        Err(e) => {
            // https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
            println!("{:#?}", e);
            // Temporay solution to make the project grow first.
            // You may build custom error Struct if necessary.
            // return Err(warp::reject::custom(UserError))
            return Err(warp::reject::not_found())
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn create(new_post: NewPost) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = new_post
        .create(&conn);

    let reply = match response {
        Ok(new_post) => {
            println!("{:#?}", &new_post);
            new_post
        },
        Err(e) => {
            // https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
            println!("{:#?}", e);
            // Temporay solution to make the project grow first.
            // You may build custom error Struct if necessary.
            // return Err(warp::reject::custom(UserError))
            return Err(warp::reject::not_found())
        }
    };
    Ok(warp::reply::json(&reply))
}

// Make UpdatePost Struct with Optional values in it if necessary.
// Use this to make all CRUD REST API work first.
pub async fn update(id: i32, update_post: NewPost) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Post::update(&id, &update_post, &conn);

    let reply = match response {
        Ok(null) => {
            println!("{:#?}", &null);
            null
        },
        Err(e) => {
            // https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
            println!("{:#?}", e);
            // Temporay solution to make the project grow first.
            // You may build custom error Struct if necessary.
            // return Err(warp::reject::custom(UserError))
            return Err(warp::reject::not_found())
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn delete(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Post::delete(&id, &conn);

    let reply = match response {
        Ok(null) => {
            println!("{:#?}", &null);
            null
        },
        Err(e) => {
            // https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
            println!("{:#?}", e);
            // Temporay solution to make the project grow first.
            // You may build custom error Struct if necessary.
            // return Err(warp::reject::custom(UserError))
            return Err(warp::reject::not_found())
        }
    };
    Ok(warp::reply::json(&reply))
}

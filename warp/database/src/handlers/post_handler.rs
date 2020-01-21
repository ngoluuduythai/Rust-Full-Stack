use warp;

use crate::models::post::{
    PostList,
    Post,
};
use crate::db_connection::{ establish_connection };

// Use this to debug and verify API chaining work or not.
// pub async fn repeat(input: String) -> Result<impl warp::Reply, warp::Rejection> {
//     println!("{:#?}", &input);
//     Ok(warp::reply::html(input))
// }

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let conn = establish_connection();
    let response = PostList::list(&conn);
    println!("{:#?}", &response);

    Ok(warp::reply::json(&response))
}

pub async fn get(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = establish_connection();
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

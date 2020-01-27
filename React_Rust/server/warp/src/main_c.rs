// Compare this(https://github.com/seanmonstar/warp/blob/master/examples/file.rs) with the entire project.

// If you want to deploy it with Docker, refer to
// https://github.com/steadylearner/Rust-Full-Stack/tree/master/React_Rust/server/actix

// If you want to modulize it, refer to
// https://github.com/steadylearner/Rust-Full-Stack/tree/master/warp/hello_world

use warp::{self, path, Filter};

use console::Style;

// How to serve React or other single page apps?
// 1. Find how to serve files in public/ or static/.
// 2. Serve production index.html file from them for speicfic routes.

#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new()
        .blue();

    // 1. $curl 0.0.0.0:8000/public/index.html
    // dir already includes GET / ...
    // (You don't have to prefix warp::get("/")" here)
    let public_files = warp::fs::dir("./public/");

    // 2. $curl 0.0.0.0:8000
    // p.s If you liked the React app here and need a help with it,
    // please contact me with https://www.linkedin.com/in/steady-learner-3151b7164/
    // (I can work with React, Node, Rust, Python, (Golang))
    let single_page_app = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./public/index.html"));

    // GET / => ./public/index.html
    // GET /public/... => ./public/..
    let routes = public_files.or(single_page_app);

    println!("\nRust Warp Server ready at {}", blue.apply_to(&target));
    println!("Use $curl http://0.0.0.0:8000 to test the end point.");

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

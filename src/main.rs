#[allow(unused)]

mod api;
mod middleware;
mod models;
use std::error::Error;
use warp::Filter;

const FOLDER: &str = "template/";

#[tokio::main]
async fn main() {
    // Define a basic route

    let pool = database::get_pool().await?; // Acquire the pool asynchronously

    let mut client = pool.acquire().await?; // Acquire a client for the operation

    let query = "SELECT * FROM users WHERE id = $1";
    let result = client.query(&query, &[&10]).await?; // Execute the query with parameter

    for row in result.iter() {
        let id: i32 = row.get(0);
        let username: &str = row.get(1);
        println!("User ID: {}, Username: {}", id, username);
    }

    let _ = client.release(); // Release the client back to the pool

    let hello = warp::path("hi").and(warp::get()).map(||"hello from hi");
    let apis = hello.or(users_filter());

    let root = warp::get().and(warp::path::end()).and(warp::fs::file(format!("{}/index.html", FOLDER)));
    let static_files = root;
      // Combine routes
      let routes = apis
        .or(static_files);

    // Start the server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8000))
        .await;

}
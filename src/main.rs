use std::net::TcpListener;

use vitalMile::{api::router::create_router, infrastructure::dbconnector};


mod api;
mod domain;

#[tokio::main]
async fn main() {
    let app = create_router();

    println!("Hello, world!");
    //let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    //axum::serve(listener, app).await.unwrap();
    dbconnector::connect_to_db().await.unwrap();
}


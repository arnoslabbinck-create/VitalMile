use std::net::TcpListener;

mod api;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
}

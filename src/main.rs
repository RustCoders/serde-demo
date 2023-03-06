// main.rs

// This is needed for CreateUser, though it's not clear why
#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use axum::{Router, extract::Json, extract::Path};
use axum::routing::{get, post};
use std::net::SocketAddr;

// Input to post, user must supply password
#[derive(Deserialize, Debug)]
struct CreateUser {
    email: String,
    password: String,
}

// Output from post.  Password is now hidden, and a user ID is added
#[derive(Serialize)]
struct User {
    email: String,
    id: u32
}

#[tokio::main]
async fn main() {
    // build our application with a get and post route
    let app = Router::new()
        .route("/user", post(create_user))
        .route("/user/:user_id", get(get_user));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_user(Path(user_id) : Path<u32>) -> Json<User>  {
    // Actually look up user from id, etc.
    Json(User{email: "everyone@ubiquitous.com".to_string(), id: user_id})
}

async fn create_user(Json(payload): Json<CreateUser>) {
    // Check for existing, save to database, etc.
    println!("Inside create_user:  {:?}", payload);
}
// users.rs

use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use warp::{Rejection, Reply};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    // You can add more claims as needed
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
}

pub async fn login_handler() -> Result<impl Reply, Rejection> {
    // Your login logic goes here
    // Generate a JWT token for the authenticated user

    let claims = key.verify_token::<NoCustomClaims>(&token, None)?;
    Ok(warp::reply::json(&token).into_response());
}

pub async fn logout_handler() -> Result<impl Reply, Rejection> {
    // Your logout logic goes here
    Ok(warp::reply::json(&"Logout successful"))
}

pub async fn get_user_handler() -> Result<impl Reply, Rejection> {
    // Your logic to retrieve a user goes here
    let user = User {
        id: 1,
        username: String::from("example_user"),
    };
    Ok(warp::reply::json(&user))
}

pub async fn create_user_handler() -> Result<impl Reply, Rejection> {
    // Your logic to create a user goes here
    Ok(warp::reply::json(&"User created"))
}

pub async fn update_user_handler() -> Result<impl Reply, Rejection> {
    // Your logic to update a user goes here
    Ok(warp::reply::json(&"User updated"))
}

pub async fn list_users_handler() -> Result<impl Reply, Rejection> {
    // Your logic to retrieve a list of users goes here
    let users = vec![
        User {
            id: 1,
            username: String::from("user1"),
        },
        User {
            id: 2,
            username: String::from("user2"),
        },
    ];
    Ok(warp::reply::json(&users))
}

fn generate_jwt_token(sub: &str) -> Result<String, jwt_simple::error::Error> {
    // Generate a JWT token with the provided subject (sub)
    let key = HS256Key::generate();
    let claims = Claims::create(Duration::from_hours(2));
    let token = key.authenticate(claims)?;
    encode(header, &claims, "your_secret_key")
}


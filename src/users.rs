// users.rs
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use warp::{Rejection, Reply};
use bcrypt::{hash, DEFAULT_COST};

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

pub async fn create_user_handler(
    user_data: warp::query::Form<UserData>,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
    // Your logic to create a user goes here
    let hashed_password = hash_password(&user_data.password)?;

    // Insert user data into the database
    insert_user(&db_pool, &user_data.username, &user_data.email, &hashed_password).await?;

    Ok(warp::reply::json(&"User created"))
}

// New struct for user data from the request
#[derive(Deserialize)]
struct UserData {
    username: String,
    email: String,
    password: String,
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
    let claims = Claims {
        sub: sub.to_string(),
    };
    let token = key.authenticate(claims)?;
    Ok(token)
}

fn authenticate() -> impl Filter<Extract = (Claims,), Error = Rejection> + Clone {
    warp::header::optional("Authorization")
        .and_then(move |authorization_header: Option<String>| {
            let token = match authorization_header {
                Some(header) => header.trim_start_matches("Bearer ").to_string(),
                None => return Ok(Err(warp::reject::custom(AuthError::MissingToken))),
            };

            // Decode and verify the JWT token
            decode::<Claims>(&token, "your_secret_key")
                .map(|claims| Ok(claims.claims))
                .map_err(|_| warp::reject::custom(AuthError::InvalidToken))
        })
}

#[derive(Debug)]
enum AuthError {
    MissingToken,
    InvalidToken,
}

impl warp::reject::Reject for AuthError {}

fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    // Generate a salted hash of the password using bcrypt
    hash(password, DEFAULT_COST)
}

async fn insert_user(
    db_pool: &DbPool,
    username: &str,
    email: &str,
    hashed_password: &str,
) -> Result<(), warp::Rejection> {
    // Connect to the database
    let client = db_pool.get().await.map_err(|e| {
        eprintln!("Database connection error: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    // Execute SQL query to insert user into the 'users' table
    let query = format!(
        "INSERT INTO users (username, email, password) VALUES ('{}', '{}', '{}')",
        username, email, hashed_password
    );

    client
        .query(&query, &[])
        .await
        .map_err(|e| {
            eprintln!("Database query error: {}", e);
            warp::reject::custom(DatabaseError)
        })?;

    Ok(())
}

// Database error type
#[derive(Debug)]
struct DatabaseError;

impl warp::reject::Reject for DatabaseError {}

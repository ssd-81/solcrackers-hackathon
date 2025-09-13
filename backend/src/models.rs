use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Represents a user row in the database.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

/// Represents the JSON payload for creating a new user.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
}

/// Represents a blog post row in the database.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
}

/// Represents the JSON payload for creating a new post.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub user_id: i32,
}

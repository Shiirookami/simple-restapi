use rocket::{Route, http::Status};
use rocket::tokio::sync::RwLock;
use std::sync::Arc;

use crate::models::user::User;

type UsersDB = Arc<RwLock<Vec<User>>>;

#[get("/users")]
pub async fn get_users(db: &rocket::State<UsersDB>) -> Result<rocket::http::Json<Vec<User>>, Status> {
    let users = db.read().await;
    Ok(rocket::http::Json(users.clone()))
}

pub fn routes() -> Vec<Route> {
    routes![get_users]
}

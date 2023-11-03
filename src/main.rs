#[macro_use]
extern crate rocket;

mod routes;
mod models;

use routes::api;

#[launch]
fn rocket() -> _ {
    let users_db: api::UsersDB = Arc::new(RwLock::new(vec![]));

    rocket::build()
        .manage(users_db)
        .mount("/api", api::routes())
}

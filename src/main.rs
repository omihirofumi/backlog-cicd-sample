#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct User {
    id: i32,
    name: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/user", data = "<user>")]
fn user(user: Json<User>) -> &'static str {
    println!("User: {:?}", user);
    "User"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, user])
}

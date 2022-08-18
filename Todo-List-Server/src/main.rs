#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket::serde::json::Json;
use rocket::tokio::time::{sleep, Duration};

use disel::prelude::*;

/// Gets the home page
#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

/// Gets all the todo items
#[get("/")]
fn list_todos() -> &'static str {
    "Hello, World!"
}

/// Get a single Todo item based on its ID
#[get("/<id>")]
fn get_todo(id: u32) -> &'static str {
    "Hello, World!"
}

/// Creates a new Todo item
#[post("/")]
fn create_todo() -> &'static str {
    "Hello, World!"
}

/// Deletes a Todo item
#[delete("/<u32>")]
fn delete_todo(id: u32) -> &'static str {
    "Hello, World!"
}

/// Updates an existing Todo item
#[put("/")]
fn update_todo() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/todo", routes![list_todos])
        .mount("/todo", routes![get_todo])
        .mount("/todo", routes![create_todo])
        .mount("/todo", routes![delete_todo])
        .mount("/todo", routes![update_todo])
        .mount("/public", FileServer::from(relative!("static/")))
}

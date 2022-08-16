#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket::response::content::Html;
use rocket::tokio::time::{sleep, Duration};
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[derive(Database)]
#[database("sqlite_logs")]
struct Todos(sqlx::SqlitePool);

#[get("/<id>")]
async fn read(mut db: Connection<Logs>, id: i64) -> Option<String> {
    sqlx::query("SELECT content FROM Todos WHERE id = ?")
        .bind(id)
        .fetch_one(&mut *db)
        .await
        .and_then(|r| Ok(r.try_get(0)?))
        .ok()
}

#[post("/todo")]
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![about])
        .mount("/", routes![delay])
        .mount("/public", FileServer::from(relative!("static/")))
}

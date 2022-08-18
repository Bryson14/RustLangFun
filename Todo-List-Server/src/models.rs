use rocket::serde::{Deserialize, Serialize};

use diesel::{AsChangeset, Insertable, Queryable};

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "todos"]
struct Todo {
    id: u32,
    title: String,
    msg: String,
    completed: bool,
}

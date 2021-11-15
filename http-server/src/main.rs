#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn about() -> &'static str {
    "<h1>Bryson</h1><p> My name is jonas !!!!!! </p>"
}

#[get("/about")]
fn about_new() -> &'static str {
    "My Name is Bryon!!. Can you believe this??"
}

#[launch]
fn rocket() -> _ {
    println!("this is a macro. OMG!");

    rocket::build()
        .mount("/", routes![index])
        .mount("/about", routes![about])
        .mount("/about", routes![about_new])
}

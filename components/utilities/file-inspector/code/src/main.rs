#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod cors;

use cors::CORS;

#[get("/")]
fn index() -> &'static str {
    "Hello, worlds!"
}

#[get("/status")]
fn status() -> &'static str {
    "OK"
}

fn main() {
    let mut server = rocket::ignite();

    server = server.attach(CORS{});
    server = server.mount("/", routes![index, status]);

    server.launch();
}
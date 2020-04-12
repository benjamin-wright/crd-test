#![feature(proc_macro_hygiene, decl_macro)]

use std::{ env, fs, io };

#[macro_use] extern crate rocket;
use rocket::response::content::{ Json };
use serde_json::{json};

mod cors;
use cors::CORS;

#[get("/status")]
fn status() -> Json<String> {
    Json(json!({ "Status": "OK" }).to_string())
}

fn get_entries<'a>(path: &'a String) -> io::Result<Vec<String>> {
    let files = fs::read_dir(path);
}

#[get("/list")]
fn list() -> Json<String> {
    let path = env::var("DATA_PATH").unwrap_or(String::from("/data"));
    let files = fs::read_dir(path);

    Json(json!({ "path": path.to_string(), "dirs": [ "dir1", "dir2" ] }).to_string())
}

fn main() {
    let mut server = rocket::ignite();

    server = server.attach(CORS{});
    server = server.mount("/", routes![status, list]);

    server.launch();
}
#![feature(proc_macro_hygiene, decl_macro)]

use std::{ env, fs, io };

#[macro_use] extern crate rocket;
use rocket::response::content::{ Json };
use serde_json::{json};

mod cors;
use cors::CORS;
use std::path::Path;

#[get("/status")]
fn status() -> Json<String> {
    Json(json!({ "Status": "OK" }).to_string())
}

fn get_entries<'a>(path: &'a str) -> io::Result<Vec<String>> {
    let path_obj = Path::new(path);

    let entries = get_entries_rec(&path_obj)?;
    let path_with_trailing_slash = format!("{}/", path);
    return Ok(entries.into_iter().map(|x| x.trim_start_matches(&path_with_trailing_slash).to_string()).collect());
}

fn get_entries_rec<'a>(path: &Path) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(path)?;
    let mut files = vec![];

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.to_str().unwrap().to_string().contains("..") {
            continue;
        }

        if path.is_dir() {
            let results = get_entries_rec(&path)?;

            for result in results {
                files.push(result);
            }
        } else {
            files.push(path.to_str().unwrap().to_string());
        }
    }

    return Ok(files);
}

#[get("/list")]
fn list() -> Json<String> {
    let path = env::var("DATA_PATH").unwrap_or(String::from("/data"));
    let files = get_entries(&path).unwrap_or(vec![]);

    Json(json!({ "path": path.to_string(), "files": files }).to_string())
}

fn main() {
    let mut server = rocket::ignite();

    server = server.attach(CORS{});
    server = server.mount("/", routes![status, list]);

    server.launch();
}
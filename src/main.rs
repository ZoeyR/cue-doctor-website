#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket::config::{Config, Environment};
#[get("/<file..>")]
fn assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).ok()
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("www/index.html").ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, assets]).launch();
}
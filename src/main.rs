#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::path::{Path, PathBuf};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use r2d2::{Config, Pool};
use r2d2_diesel::ConnectionManager;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::JSON;

mod frontend;
mod models;
mod schema;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(FromForm)]
struct ProductId {
    id: i32,
}
// paths needed
// put orders
// get order

#[get("/<file..>")]
fn assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).ok()
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("www/index.html").ok()
}

#[post("/orders", format = "application/json", data = "<order>")]
fn new_order(order: JSON<frontend::Order>, db: State<DbPool>) -> &'static str {
    unimplemented!()
}

#[get("/products?<product_id>")]
fn products(product_id: ProductId, db: State<DbPool>) -> Result<JSON<frontend::Product>, ()> {
    use schema::products::dsl::*;

    let db = db.inner()
        .get()
        .map_err(|_| ())?;
    let product = products.filter(id.eq(product_id.id))
        .first::<models::Product>(&*db)
        .map_err(|_| ())?;

    Ok(JSON(product.into()))
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let r2d2_config = Config::default();
    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool =
        Pool::new(r2d2_config, connection_manager).expect("Failed to created connection pool.");

    rocket::ignite().mount("/", routes![index, assets, products]).manage(pool).launch();
}

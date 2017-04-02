#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::path::{Path, PathBuf};

use errors::*;
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
mod errors;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(FromForm)]
struct ProductId {
    id: i32,
}

#[derive(FromForm)]
struct OrderId {
    id: i32,
}
// paths needed
// put orders
// get order

#[get("/static/<file..>")]
fn assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).ok()
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("www/index.html").ok()
}

#[post("/orders", format = "application/json", data = "<order>")]
fn new_order(order: JSON<frontend::Order>, db: State<DbPool>) -> Result<JSON<i32>> {
    use schema::*;

    let db = db.inner().get()?;

    let new_order = models::NewOrder { address: &order.0.address };
    let model_order = diesel::insert(&new_order).into(orders::table)
        .get_result::<models::Order>(&*db)?;

    let new_items_iter = order.0
        .items
        .into_iter()
        .map(|item| {
            models::NewOrderItem {
                order_id: model_order.id,
                product_id: item.product.id,
                quantity: item.quantity,
            }
        })
        .collect::<Vec<_>>();

    diesel::insert(&new_items_iter).into(order_items::table)
        .get_results::<models::OrderItem>(&*db)?;

    Ok(JSON(model_order.id))
}

#[post("/orders?<order_id>")]
fn get_order(order_id: OrderId, db: State<DbPool>) -> Result<JSON<frontend::Order>> {
    use schema::*;

    let db = db.inner().get()?;
    let order = orders::table.filter(orders::id.eq(order_id.id)).first::<models::Order>(&*db)?;
    let model_items = order_items::table.inner_join(products::table)
        .filter(order_items::id.eq(order_id.id))
        .load(&*db)?;

    let frontend_items = model_items.into_iter()
        .map(|(item, product): (models::OrderItem, _)| {
                 frontend::OrderItem {
                     product: product,
                     quantity: item.quantity,
                 }
             })
        .collect();
    Ok(JSON(frontend::Order {
                items: frontend_items,
                address: order.address,
            }))
}

#[get("/products?<product_id>")]
fn products(product_id: ProductId, db: State<DbPool>) -> Result<JSON<frontend::Product>> {
    use schema::products::dsl::*;

    let db = db.inner().get()?;
    let product = products.filter(id.eq(product_id.id)).first::<models::Product>(&*db)?;

    Ok(JSON(product.into()))
}

#[get("/products")]
fn all_products(db: State<DbPool>) -> Result<JSON<Vec<frontend::Product>>> {
    use schema::products::dsl::*;

    let db = db.inner().get()?;
    let products_res = products.load::<models::Product>(&*db)?;

    Ok(JSON(products_res))
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let r2d2_config = Config::default();
    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool =
        Pool::new(r2d2_config, connection_manager).expect("Failed to created connection pool.");

    rocket::ignite()
        .mount("/",
               routes![index, assets, products, all_products, new_order, get_order])
        .manage(pool)
        .launch();
}

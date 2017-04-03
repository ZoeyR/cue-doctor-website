extern crate serde_json;

use std::env;

use super::{dotenv, rocket, DbPool};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket::testing::MockRequest;
use rocket::http::{Status, Method, ContentType};
use self::serde_json::from_str;

#[test]
fn all_products() {
    let test_db = DbPool::Test(test_connection);
    let rocket = rocket::ignite().mount("/", routes![super::all_products]).manage(test_db);

    let mut req = MockRequest::new(Method::Get, "/products");
    let mut res = req.dispatch_with(&rocket);

    assert_eq!(res.status(), Status::Ok);
    let products: Vec<super::frontend::Product> = res.body()
        .and_then(|b| b.into_string())
        .and_then(|body_str| from_str(&body_str).ok())
        .unwrap();

    assert_eq!(products.len(), 4);
}

#[test]
fn one_product() {
    let test_db = DbPool::Test(test_connection);
    let rocket = rocket::ignite().mount("/", routes![super::products]).manage(test_db);

    let mut req = MockRequest::new(Method::Get, "/products?id=1");
    let mut res = req.dispatch_with(&rocket);

    assert_eq!(res.status(), Status::Ok);
    let product: super::frontend::Product = res.body()
        .and_then(|b| b.into_string())
        .and_then(|body_str| from_str(&body_str).ok())
        .unwrap();

    assert_eq!(product.name, "Cue Wax");
}

#[test]
fn new_order() {
    use super::frontend::{Order, OrderItem, Product};
    let test_db = DbPool::Test(test_connection);
    let order = Order {
        items: vec![OrderItem {
                        product: Product {
                            id: 1,
                            name: "Cue Wax".into(),
                            description: "TODO".into(),
                            price: 550,
                        },
                        quantity: 3,
                    }],
        address: "None".into(),
    };
    let rocket = rocket::ignite().mount("/", routes![super::new_order]).manage(test_db);

    let mut req = MockRequest::new(Method::Post, "/orders")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&order).unwrap());
    let res = req.dispatch_with(&rocket);

    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn get_order() {
    use super::frontend::{Order, OrderItem, Product};
    let test_db = DbPool::Test(test_connection_with_order);
    let order = Order {
        items: vec![OrderItem {
                        product: Product {
                            id: 1,
                            name: "Cue Wax".into(),
                            description: "TODO".into(),
                            price: 550,
                        },
                        quantity: 3,
                    },
                    OrderItem {
                        product: Product {
                            id: 2,
                            name: "Shaft Cleaner".into(),
                            description: "TODO".into(),
                            price: 850,
                        },
                        quantity: 4,
                    }],
        address: "None".into(),
    };
    let rocket = rocket::ignite().mount("/", routes![super::get_order]).manage(test_db);

    let mut req = MockRequest::new(Method::Get, "/orders?id=1");
    let mut res = req.dispatch_with(&rocket);

    let res_order: Order = res.body()
        .and_then(|b| b.into_string())
        .and_then(|body_str| from_str(&body_str).ok())
        .unwrap();
    assert_eq!(order, res_order);
}

fn test_connection() -> PgConnection {
    dotenv().ok();
    let connection_url = env::var("DATABASE_TEST_URL")
        .expect("DATABASE_TEST_URL must be set in order to run tests");
    let conn = ::diesel::pg::PgConnection::establish(&connection_url).unwrap();
    conn.begin_test_transaction().unwrap();

    conn
}

fn test_connection_with_order() -> PgConnection {
    dotenv().ok();
    let connection_url = env::var("DATABASE_TEST_URL")
        .expect("DATABASE_TEST_URL must be set in order to run tests");
    let conn = ::diesel::pg::PgConnection::establish(&connection_url).unwrap();
    conn.begin_test_transaction().unwrap();
    conn.execute("INSERT INTO orders (id, address) VALUES (1, 'None')").unwrap();
    conn.execute("INSERT INTO order_items (id, order_id, product_id, quantity) VALUES (1, 1, 1, \
                  3)")
        .unwrap();
    conn.execute("INSERT INTO order_items (id, order_id, product_id, quantity) VALUES (2, 1, 2, \
                  4)")
        .unwrap();
    conn.execute("INSERT INTO order_items (id, order_id, product_id, quantity) VALUES (3, 2, 2, \
                  5)")
        .unwrap();

    conn
}
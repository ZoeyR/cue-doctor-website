#[derive(Queryable)]
pub struct Order {
    id: i32,
    address: String,
}

#[derive(Queryable)]
pub struct OrderItem {
    id: i32,
    order_id: i32,
    product_id: i32,
    quantity: i32,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Product {
    id: i32,
    name: String,
    description: String,
    price: i32,
}
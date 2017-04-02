use schema::*;

#[derive(Associations, Identifiable, Queryable)]
#[has_many(order_items)]
pub struct Order {
    pub id: i32,
    pub address: String,
}

#[derive(Associations, Identifiable, Queryable)]
#[belongs_to(Order)]
#[belongs_to(Product)]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Associations, Identifiable, Serialize, Deserialize, Queryable)]
#[has_many(order_items)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: i32,
}
use schema::*;

#[derive(Associations, Identifiable, Queryable)]
#[has_many(order_items)]
pub struct Order {
    pub id: i32,
    pub address: String,
}

#[derive(Insertable)]
#[table_name="orders"]
pub struct NewOrder<'a> {
    pub address: &'a str,
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

#[derive(Insertable)]
#[table_name="order_items"]
pub struct NewOrderItem {
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Associations, Identifiable, Serialize, Deserialize, Debug, Queryable, Eq, PartialEq)]
#[has_many(order_items)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: i32,
}
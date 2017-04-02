#[derive(Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Order {
    pub items: Vec<OrderItem>,
    pub address: String,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct OrderItem {
    pub product: Product,
    pub quantity: i32,
}

pub use models::Product;
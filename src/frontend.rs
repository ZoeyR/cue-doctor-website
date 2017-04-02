#[derive(Serialize, Deserialize)]
pub struct Order {
    pub items: Vec<OrderItem>,
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct OrderItem {
    pub product: Product,
    pub quantity: i32,
}

pub use models::Product;
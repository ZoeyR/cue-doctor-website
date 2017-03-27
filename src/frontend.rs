#[derive(Serialize, Deserialize)]
pub struct Order {
    pub items: Vec<Product>,
    pub address: String,
}

pub use models::Product;
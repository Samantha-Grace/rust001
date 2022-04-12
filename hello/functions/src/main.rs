use serde_json;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct OrderResponse {
    orders: Vec<Order>,
}
#[derive(Serialize, Deserialize)]
struct Order {
    certificate_id: String,
    order_id: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let order_str = fs::read_to_string("data.json")?;
    let o: OrderResponse = serde_json::from_str(&order_str)?;
    let filtered_array: Vec<&Order> = o.orders.iter().filter(|x| x.certificate_id == "").collect();

    println!("Orders: {}", o.orders.len());
    println!("Filt Array: {}", filtered_array.len());
    Ok(())
}

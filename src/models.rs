use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Price {
    max_price: i32,
    price: i32,
}
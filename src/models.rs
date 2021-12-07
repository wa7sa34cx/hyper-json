use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Price {
    pub max_price: i32,
    pub price: i32,
}

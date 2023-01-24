#[derive(Debug)]
#[derive(Clone)]
pub struct Order {
    pub amount: u64,
    pub price: u64,
    pub id : String,
    pub side : i8
}

impl Order {
    pub fn default() -> Order {
        Order {
            amount: 0,
            price: 0,
            id: String::from(""),
            side: 0
        }
    }
}
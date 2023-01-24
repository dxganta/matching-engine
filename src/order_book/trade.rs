pub struct Trade {
    pub taker_order_id : String,
    pub maker_order_id : String,
    pub amount : u64,
    pub price : u64,
}

impl Trade {
    pub fn new(taker_order_id:String, maker_order_id:String, amount:u64, price:u64) -> Trade {
        Trade {
            taker_order_id,
            maker_order_id,
            amount,
            price
        }
    }
}
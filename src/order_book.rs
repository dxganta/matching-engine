pub mod order;
pub mod trade;

pub use order::Order;

pub struct OrderBook {
    pub buy_orders : Vec<Order>, // buy orders are in ascending order
    pub sell_orders : Vec<Order> // sell orders are in descending order
}

impl OrderBook {
    // add a buy order to the order book
    fn addBuyOrder(&mut self, order:Order) {
        // number of buy orders
        let n = self.buy_orders.len();

        for i in (0..n-1).rev() {
            if order.price > self.buy_orders[i].price {
                self.buy_orders.insert(i+1, order);
                break;
            } else {
                self.buy_orders.insert(0, order);
            }
        }
    }   

    // add a sell order to the order book
    fn addSellOrder(&mut self, order:Order) {
        // number of sell orders
        let n = self.sell_orders.len();

        for i in (0..n-1).rev() {
            if order.price < self.sell_orders[i].price  {
                self.sell_orders.insert(i+1, order);
                break;
            } else {
                self.sell_orders.insert(0, order);
            }
        }
    }

    // remove a buy order from the order book at index ind
    fn removeBuyOrder(&mut self, ind:usize) {
        self.buy_orders.remove(ind);
    }

    // remove a sell order from the order book at index ind
    fn removeSellOrder(&mut self, ind:usize) {
        self.sell_orders.remove(ind);
    }

    // process a limit buy order
    // fn processLimitBuyOrder(&self, order:Order) {}
}
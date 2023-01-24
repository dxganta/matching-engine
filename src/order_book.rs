pub mod order;
pub mod trade;

pub use order::Order;
pub use trade::Trade;

pub struct OrderBook {
    pub buy_orders : Vec<Order>, // buy orders are in ascending order
    pub sell_orders : Vec<Order> // sell orders are in descending order
}

impl OrderBook {
    // add a buy order to the order book
    fn add_buy_order(&mut self, order:Order) {
        // number of buy orders
        let n = self.buy_orders.len();

        for i in (0..n-1).rev() {
            if order.price > self.buy_orders[i].price {
                self.buy_orders.insert(i+1, order.clone());
                break;
            } else {
                self.buy_orders.insert(0, order.clone());
            }
        }
    }   

    // add a sell order to the order book
    fn add_sell_order(&mut self, order:Order) {
        // number of sell orders
        let n = self.sell_orders.len();

        for i in (0..n-1).rev() {
            if order.price < self.sell_orders[i].price  {
                self.sell_orders.insert(i+1, order.clone());
                break;
            } else {
                self.sell_orders.insert(0, order.clone());
            }
        }
    }

    // remove a buy order from the order book at index ind
    fn remove_buy_order(&mut self, ind:usize) {
        self.buy_orders.remove(ind);
    }

    // remove a sell order from the order book at index ind
    fn remove_sell_order(&mut self, ind:usize) {
        self.sell_orders.remove(ind);
    }

    // process a limit buy order
    fn process_limit_buy(&mut self, mut order:Order) -> Vec<Trade> {
        let mut trades:Vec<Trade> = Vec::new();
        let n = self.sell_orders.len();

        // check if we have atleast one matching order
        if n != 0 || self.sell_orders[n-1].price <= order.price {
            // traverse all orders that match
            for i in (0..n-1).rev() {
                let sell_order: Order = self.sell_orders[i].clone();

                if sell_order.price > order.price {
                    break;
                }

                // fill the entire order
                if sell_order.amount >= order.amount {
                    trades.push(Trade::new(order.clone().id, sell_order.id, order.amount, order.price));
                    self.sell_orders[i].amount -= order.amount;
                    if self.sell_orders[i].amount == 0 {
                        self.remove_sell_order(i);
                    }

                    return trades;
                }

                // partially fill the order
                if sell_order.amount < order.amount {
                    trades.push(Trade::new(order.clone().id, sell_order.id, sell_order.amount, order.price));
                    order.amount -= sell_order.amount;
                    self.remove_sell_order(i);
                }
            }
        }

        self.add_buy_order(order);
        return trades
    }
}
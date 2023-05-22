use super::order::Limit;
use super::OrderSide;
use super::Price;
use std::collections::HashMap;
use std::println;

pub struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn add_limit_order(&mut self, price: Price, size: f64, side: OrderSide) {
        match side {
            OrderSide::Bid => {
                let limit = self.asks.get_mut(&price);
                match limit {
                    Some(limit) => println!("Got limit"),
                    None => {
                        let mut limit = Limit::new(price);
                    }
                }
            }
            OrderSide::Ask => {
                let limit = self.bids.get_mut(&price);
            }
        }
    }
}

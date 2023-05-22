use super::order::Limit;
use super::OrderSide;
use super::Price;
use std::collections::HashMap;

pub struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
    decimals: u64,
}

impl OrderBook {
    pub fn add_limit_order(&mut self, price: f64, size: f64, side: OrderSide) {
        match side {
            OrderSide::Bid => {
                let price = Price::new(price, self.decimals);
            }
            OrderSide::Ask => {
                let price = Price::new(price, self.decimals);
            }
        }
    }
}

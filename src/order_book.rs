#![allow(dead_code)]
use crate::order::Order;

use super::order::Limit;
use super::OrderSide;
use super::Price;
use std::collections::HashMap;
use std::println;

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
    decimals: u64,
}

impl OrderBook {
    pub fn new(decimals: u64) -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
            decimals,
        }
    }

    pub fn add_limit_order(&mut self, price: f64, size: f64, side: OrderSide) {
        let order_price = Price::new(price, self.decimals);
        let order = Order::new(OrderSide::Bid, size);
        match side {
            OrderSide::Bid => {
                let limit = self.bids.get_mut(&order_price);
                match limit {
                    Some(limit) => limit.add_order(order),
                    None => {
                        let mut limit = Limit::new(&order_price);
                        limit.add_order(order);
                        self.bids.insert(order_price, limit);
                    }
                };
            }
            OrderSide::Ask => {
                let limit = self.asks.get_mut(&order_price);
                match limit {
                    Some(limit) => limit.add_order(order),
                    None => {
                        let mut limit = Limit::new(&order_price);
                        limit.add_order(order);
                        self.asks.insert(order_price, limit);
                    }
                }
            }
        }
    }
}

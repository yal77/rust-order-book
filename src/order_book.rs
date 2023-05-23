#![allow(dead_code)]
use crate::order::Order;

use super::order::Limit;
use super::OrderSide;
use ordered_float::OrderedFloat;
use std::collections::HashMap;

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<OrderedFloat<f64>, Limit>,
    bids: HashMap<OrderedFloat<f64>, Limit>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn add_limit_order(&mut self, price: f64, order: Order) {
        let order_price = OrderedFloat(price);
        match order.side {
            OrderSide::Bid => {
                let limit = self.bids.get_mut(&order_price);
                match limit {
                    Some(limit) => limit.add_order(order),
                    None => {
                        let mut limit = Limit::new(order_price);
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
                        let mut limit = Limit::new(order_price);
                        limit.add_order(order);
                        self.asks.insert(order_price, limit);
                    }
                }
            }
        }
    }

    pub fn get_best_ask(&self) -> f64 {
        let mut min = OrderedFloat(f64::INFINITY);
        for (_k, v) in &self.asks {
            let price = v.price;
            if price < min {
                min = v.price;
            }
        }
        min.into_inner()
    }
    pub fn get_best_bid(&self) -> f64 {
        let mut max = OrderedFloat(0.0);
        for (_k, v) in &self.asks {
            let price = v.price;
            if price > max {
                max = v.price;
            }
        }
        max.into_inner()
    }

    pub fn get_mid_price(&self) -> f64 {
        (self.get_best_ask() + self.get_best_bid()) / 2.0
    }
}

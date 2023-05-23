#![allow(dead_code)]
use crate::order::Order;
use crate::MarketOrderSide;

use super::order::Limit;
use super::OrderSide;
use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::println;

use itertools::Itertools;
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

    pub fn add_market_order(&mut self, size: f64, order_side: MarketOrderSide) -> f64 {
        if size < 0.0 {
            return 0.0;
        }
        match order_side {
            MarketOrderSide::Buy => {
                let best_ask = self.get_best_ask();
                let limit = self.asks.get_mut(&OrderedFloat(best_ask));
                let rem = match limit {
                    Some(limit) => limit.fill_market_order(size),
                    None => {
                        println!("No Liquidity Avaliable");
                        return size;
                    }
                };
                if rem.1 <= 0.0 {
                    self.asks.remove(&OrderedFloat(best_ask));
                }
                if rem.0 > 0.0 {
                    self.asks.remove(&OrderedFloat(best_ask));
                    return self.add_market_order(rem.0, order_side);
                }
            }
            MarketOrderSide::Sell => {
                let best_bid = self.get_best_bid();
                let limit = self.bids.get_mut(&OrderedFloat(best_bid));
                let rem = match limit {
                    Some(limit) => limit.fill_market_order(size),
                    None => {
                        println!("No Liquidity Avaliable, Remaining: {}", size);
                        return size;
                    }
                };
                if rem.1 <= 0.0 {
                    self.bids.remove(&OrderedFloat(best_bid));
                }
                if rem.0 > 0.0 {
                    self.bids.remove(&OrderedFloat(best_bid));
                    return self.add_market_order(rem.0, order_side);
                }
            }
        }
        return 0.0;
    }

    // Turn these to Option<f64>
    pub fn get_best_ask(&self) -> f64 {
        let mut min = OrderedFloat(f64::INFINITY);
        for (_k, v) in &self.asks {
            let price = v.get_price();
            if price < min {
                min = v.get_price();
            }
        }
        min.into_inner()
    }
    pub fn get_best_bid(&self) -> f64 {
        let mut max = OrderedFloat(0.0);
        for (_k, v) in &self.bids {
            let price = v.get_price();
            if price > max {
                max = v.get_price();
            }
        }
        max.into_inner()
    }

    pub fn get_mid_price(&self) -> f64 {
        (self.get_best_ask() + self.get_best_bid()) / 2.0
    }

    pub fn print_orderbook(&self) {
        for key in self.asks.keys().sorted().rev() {
            let size = self.asks.get(key).unwrap().get_size();
            println!("{} - {} ", key, size);
        }
        println!("------------ {}", self.get_mid_price());
        for key in self.bids.keys().sorted() {
            let size = self.bids.get(key).unwrap().get_size();
            println!("{} - {} ", key, size);
        }
    }
}

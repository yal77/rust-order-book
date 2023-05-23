use std::collections::VecDeque;
use std::println;

use super::OrderSide;
use ordered_float::OrderedFloat;
use queues;
use queues::IsQueue;
use queues::Queue;

#[derive(Debug, Clone)]
pub struct Order {
    pub side: OrderSide,
    pub size: f64,
}

impl Order {
    pub fn new(side: OrderSide, size: f64) -> Order {
        Order { side, size }
    }
}

#[derive(Debug)]
pub struct Limit {
    price: OrderedFloat<f64>,
    orders: VecDeque<Order>,
}

impl Limit {
    pub fn new(price: OrderedFloat<f64>) -> Limit {
        Limit {
            price,
            orders: VecDeque::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push_back(order);
    }

    pub fn fill_market_order(&mut self, mut size: f64) -> f64 {
        if size <= 0.0 {
            return 0.0;
        };
        let next_order = self.orders.front_mut();
        match next_order {
            Some(order) => {
                if order.size > size {
                    order.size = order.size - size;
                    return 0.0;
                } else {
                    size = size - order.size;
                    match self.orders.pop_front() {
                        Some(_) => (),
                        None => println!("Error: Cannot remove item from queue!"),
                    };
                    return self.fill_market_order(size);
                }
            }
            None => {
                return size;
            }
        }
    }

    pub fn get_size(&self) -> usize {
        self.orders.len()
    }

    pub fn get_price(&self) -> OrderedFloat<f64> {
        self.price
    }
}

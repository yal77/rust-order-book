use super::OrderSide;
use ordered_float::OrderedFloat;
use queues;
use queues::IsQueue;
use queues::Queue;

#[derive(Debug, Clone)]
pub struct Order {
    pub side: OrderSide,
    size: f64,
}

impl Order {
    pub fn new(side: OrderSide, size: f64) -> Order {
        Order { side, size }
    }
}

#[derive(Debug)]
pub struct Limit {
    pub price: OrderedFloat<f64>,
    orders: queues::Queue<Order>,
}

impl Limit {
    pub fn new(price: OrderedFloat<f64>) -> Limit {
        Limit {
            price,
            orders: Queue::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        let res = self.orders.add(order);
        match res {
            Err(err) => println!("Error in adding order: {}", err),
            _ => (),
        }
    }
}

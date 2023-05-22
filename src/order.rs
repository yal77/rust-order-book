use super::OrderSide;
use super::Price;
use queues;
use queues::queue;
use queues::IsQueue;
use queues::Queue;

#[derive(Debug, Clone)]
pub struct Order {
    side: OrderSide,
    size: f64,
}

impl Order {
    pub fn new(side: OrderSide, size: f64) -> Order {
        Order { side, size }
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Price,
    orders: queues::Queue<Order>,
}

impl Limit {
    pub fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Queue::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.add(order);
    }
}

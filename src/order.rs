use super::OrderSide;
use super::Price;

#[derive(Debug)]
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
    orders: Vec<Order>,
}

impl Limit {
    pub fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

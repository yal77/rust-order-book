use super::OrderSide;
use super::Price;

pub struct Order {
    side: OrderSide,
    size: f64,
}

impl Order {
    fn new(side: OrderSide, size: f64) -> Order {
        Order { side, size }
    }
}

pub struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    pub fn new(price: f64, scalar: u64) -> Limit {
        Limit {
            price: Price::new(price, scalar),
            orders: Vec::new(),
        }
    }
}

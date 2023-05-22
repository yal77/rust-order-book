use rust_order_book::{
    order::{Limit, Order},
    order_book::{self, OrderBook},
    OrderSide, Price,
};

fn main() {
    //let orderbook = order_book::OrderBook;
    let price = Price::new(64.2, 100000);
    let mut limit = Limit::new(price);
    let order = Order::new(OrderSide::Bid, 1.25);
    limit.add_order(order);

    let orderbook = OrderBook::new();

    println!("{:?}", limit);
}

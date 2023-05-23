use std::println;

use rust_order_book::{
    order::{Limit, Order},
    order_book::OrderBook,
    OrderSide, Price,
};

fn main() {
    //let orderbook = order_book::OrderBook;
    let price = Price::new(64.2, 100000);
    println!("{}", price.to_float());
    let mut limit = Limit::new(&price.clone());
    let order = Order::new(OrderSide::Bid, 1.25);
    limit.add_order(order);

    let mut orderbook = OrderBook::new(100);
    let order = Order::new(OrderSide::Bid, 5.0);
    orderbook.add_limit_order(100.1, order);
    let order = Order::new(OrderSide::Ask, 3.0);
    orderbook.add_limit_order(100.3, order);
    let order = Order::new(OrderSide::Ask, 2.33);
    orderbook.add_limit_order(100.3, order);
    println!("{:#?}", orderbook);
}

use std::println;

use rust_order_book::{order::Order, order_book::OrderBook, OrderSide};

fn main() {
    let mut orderbook = OrderBook::new();
    let order = Order::new(OrderSide::Bid, 5.0);
    orderbook.add_limit_order(100.1, order);
    let order = Order::new(OrderSide::Ask, 3.0);
    orderbook.add_limit_order(100.2, order);
    let order = Order::new(OrderSide::Ask, 2.33);
    orderbook.add_limit_order(100.7, order);
    let order = Order::new(OrderSide::Ask, 2.6767);
    let bestask = orderbook.get_best_ask();
    orderbook.add_limit_order(bestask, order);
    println!("Best ask {}", bestask);
    println!("{:#?}", orderbook);
}

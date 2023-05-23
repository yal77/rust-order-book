use std::println;

use rust_order_book::{order::Order, order_book::OrderBook, MarketOrderSide, OrderSide};

fn main() {
    let mut orderbook = OrderBook::new();
    let order = Order::new(OrderSide::Bid, 5.0);
    orderbook.add_limit_order(100.1, order);
    let order = Order::new(OrderSide::Ask, 3.0);
    orderbook.add_limit_order(100.2, order);
    let order = Order::new(OrderSide::Ask, 2.33);
    orderbook.add_limit_order(100.7, order);
    let order = Order::new(OrderSide::Ask, 2.0);
    let bestask = orderbook.get_best_ask();
    orderbook.add_limit_order(bestask, order);
    println!("{:#?}", orderbook);
    orderbook.add_market_order(2.5, MarketOrderSide::Buy);
    orderbook.add_market_order(2.5, MarketOrderSide::Buy);
    println!("Best ask {}", bestask);
    println!("{:#?}", orderbook);
}

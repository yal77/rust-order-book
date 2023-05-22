use rust_order_book::{
    order::{Limit, Order},
    order_book::OrderBook,
    OrderSide, Price,
};

fn main() {
    //let orderbook = order_book::OrderBook;
    let price = Price::new(64.2, 100000);
    let mut limit = Limit::new(&price.clone());
    let order = Order::new(OrderSide::Bid, 1.25);
    limit.add_order(order);

    let mut orderbook = OrderBook::new(100);
    orderbook.add_limit_order(100.1, 2.0, OrderSide::Bid);
    orderbook.add_limit_order(100.2, 5.0, OrderSide::Bid);

    orderbook.add_limit_order(100.3, 5.0, OrderSide::Ask);
    orderbook.add_limit_order(100.3, 3.0, OrderSide::Ask);
    println!("{:#?}", orderbook);
}

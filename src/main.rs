use rust_order_book::{order_book, Price};

fn main() {
    //let orderbook = order_book::OrderBook;
    let price = Price::new(64.2, 100000);
    println!("{:?}", price);
}

#![allow(dead_code)]

pub mod order;
pub mod order_book;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderSide {
    Bid,
    Ask,
}

pub enum MarketOrderSide {
    Buy,
    Sell,
}

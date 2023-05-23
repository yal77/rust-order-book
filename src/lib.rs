#![allow(dead_code)]

pub mod order_book;

pub mod order;

#[derive(Debug, Clone)]
pub enum OrderSide {
    Bid,
    Ask,
}

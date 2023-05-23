#![allow(dead_code)]
pub mod order_book;

pub mod order;

#[derive(Debug, Clone)]
pub enum OrderSide {
    Bid,
    Ask,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    pub fn to_float(&self) -> f64 {
        self.integral as f64 + ((self.fractional as f64) / (self.scalar as f64))
    }

    pub fn new(price: f64, scalar: u64) -> Price {
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            scalar,
            integral,
            fractional,
        }
    }
    pub fn clone(&self) -> Price {
        Price {
            integral: self.integral,
            scalar: self.scalar,
            fractional: self.fractional,
        }
    }
}

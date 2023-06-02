use std::assert_eq;

use ordered_float::OrderedFloat;

use crate::{order::*, order_book::*, MarketOrderSide, OrderSide};

#[test]
fn order_getters() {
    let order = Order::new(OrderSide::Bid, 22.2);
    assert_eq!(order.get_side(), OrderSide::Bid);
    assert_eq!(order.get_size(), 22.2);
}

#[test]
fn limit_getsize() {
    let order1 = Order::new(OrderSide::Bid, 2.2);
    let order2 = Order::new(OrderSide::Bid, 2.3);
    let mut limit = Limit::new(OrderedFloat(5.5));
    limit.add_order(order1);
    limit.add_order(order2);
    assert_eq!(
        limit.get_size(),
        2.2 + 2.3,
        "Limit::get_size() not working as expected, value was {} instead of 4.5",
        limit.get_size()
    );
}

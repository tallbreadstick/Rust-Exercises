/*

    2 - MARKETPLACE

    Simulate a marketplace by creating an abstract data handler. Define a struct [Marketplace] with whatever fields
    you deem necessary.

    Implement the following methods:

    pub fn init() -> Self;
    > Creates a new marketplace object.

    pub fn buy(&mut self, product: &'a str, quantity: usize) -> Option<i32>;
    > Simulate a purchase of the given product of a specified quantity. Purchases must be consumed from currently existing
    > sell-orders, prioritizing the cheapest products. If there are no sell-orders for the product, there are not enough
    > sell-orders for the product, or the product does not exist, ignore the request. Otherwise, complete the request and
    > return the total amount spent on the purchase.

    pub fn sell(&mut self, product: &'a str, quantity: usize) -> Option<i32>;
    > Simulate a sale of the given product of a specified quantity. Sales must be consumed from currently existing buy-orders,
    > prioritizing the highest-paying ones. If there are no buy-orders for the product, there are not enough buy orders for the
    > product, or the product does not exist, ignore the request. Otherwise, complete the sale and return the total amount
    > earned from the sale.

    pub fn buy_order(&mut self, product: &'a str, quantity: usize, price: i32);
    > Create a buy-order for a product for a given quantity, with a given price per unit of the product.

    pub fn sell_order(&mut self, product: &'a str, quantity: usize, price: i32);
    > Create a sell-order for a product for a given quantity, with a given price per unit of the product.

    If you want a more concrete understanding of this marketplace system, you can learn about the live player-driven market
    of the MMORPG Albion Online.

*/

use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

pub struct Marketplace<'a> {
    // add the fields here...
    buy_orders: HashMap<&'a str, (usize, BinaryHeap<Order>)>,
    sell_orders: HashMap<&'a str, (usize, BinaryHeap<Reverse<Order>>)>
}

impl<'a> Marketplace<'a> {
    // implement the methods here...
}

/*

    The following struct and implementation is recommended for solving the problem using a BinaryHeap<Order> (max heap) for
    buy-orders and BinaryHeap<Reverse<Order>> (min heap) for sell-orders.

*/

#[derive(Eq, PartialEq, PartialOrd)]
struct Order {
    amount: usize,
    price: i32
}

impl Order {
    pub fn new(amount: usize, price: i32) -> Self {
        Self { amount, price }
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.price.cmp(&other.price)
    }
}
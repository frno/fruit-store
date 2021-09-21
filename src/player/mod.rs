use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::fruit::Fruit;

pub(crate) struct Player{
    cash: u32,
    inventory: HashMap<Fruit,u32>
}

impl Player{
    pub(crate) fn new(starting_cash :u32)->Player{
        let mut inventory: HashMap<Fruit,u32> = Default::default();
        for fruit in Fruit::iter() {
            inventory.insert(fruit,0);
        }
        Player{ cash: starting_cash, inventory }
    }

    /// increment user inventory of a particular fruit by 1 while decrementing cash accordingly.
    /// returns None if user does not have enough cash else returns same parameters as given
    pub(crate) fn buy_offer(&mut self, fruit :Fruit, price :u32) -> Option<(Fruit,u32)> {
        if price > self.cash{
            return None;
        }
        self.cash = self.cash-price;
        let amount = self.get_amount_of_fruit(fruit);
        let new_amount = amount+1;
        self.inventory.insert(fruit,new_amount);
        return Some((fruit, price));
    }

    /// decrement user inventory of a particular fruit by 1 while increasing cash accordingly.
    /// returns None if user does not have any inventory of the particular fruit
    pub(crate) fn sell_offer(&mut self, fruit :Fruit, price :u32) -> Option<(Fruit,u32)> {
        let amount = self.get_amount_of_fruit(fruit);
        if amount <= 0 {
            return None;
        }
        self.cash = self.cash+price;
        let new_amount = amount-1;
        self.inventory.insert(fruit,new_amount);
        return Some((fruit, price));
    }

    pub(crate) fn get_cash(&self)->u32{
        return self.cash;
    }

    pub(crate) fn get_amount_of_fruit(&self, fruit:Fruit)->u32{
        match self.inventory.get(&fruit) {
            None => {
                0
            }
            Some(amount) => {
                *amount
            }
        }
    }
}
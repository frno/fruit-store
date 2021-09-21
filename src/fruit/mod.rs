pub mod fruit_store;
pub mod fruit_offer_controller;

use rand::{distributions::{Distribution, Standard}, Rng};
use std::time::SystemTime;
use strum_macros::EnumIter;

/// Fruit offer consists of only 1 fruit and 1 price
/// Offers will expire on duration_ms
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct FruitOffer{
    fruit :Fruit,
    price :u32,
    start :SystemTime,
    duration_ms :u32,
    id :u32
}

impl FruitOffer {
    pub(crate) fn get_fruit(&self)->Fruit{
        return self.fruit.clone();
    }

    pub(crate) fn get_price(&self) -> u32 {
        return self.price;
    }

    pub(crate) fn millis_since_offer(&self) -> u32 {
        return self.start.elapsed().unwrap().as_millis() as u32;
    }

    pub(crate) fn get_duration_as_percent_of_elapsed(&self, elapsed: u32) -> f64 {
        let left;
        if self.duration_ms < elapsed {
            left = 1;
        }else{
            left = self.duration_ms - elapsed;
        }
        return (left as f64 / self.duration_ms as f64)*100f64;
    }

    pub(crate) fn get_id(&self)->u32{
        return self.id;
    }
}

/// Five classic fruits, each with their own price range determined in FruitStore::range_for_fruit
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq)]
pub(crate) enum Fruit {
    Apples,
    Banana,
    Coconut,
    DragonFruit,
    Elderberry
}

/// Create a random fruit by using rand::random()
/// # Examples
/// '''
/// let fruit :Fruit = rand::random();
/// '''
impl Distribution<Fruit> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Fruit {
        match rng.gen_range(0..=4) {
            0 => Fruit::Apples,
            1 => Fruit::Banana,
            2 => Fruit::Coconut,
            3 => Fruit::DragonFruit,
            _ => Fruit::Elderberry,
        }
    }
}


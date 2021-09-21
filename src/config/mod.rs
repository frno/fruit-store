use crate::fruit::Fruit;
use std::ops::Range;

pub(crate) struct Config{}

/// Static configuration settings
impl Config {
    /// How much cash a player should start with
    pub(crate) const STARTING_CASH: u32 = 50;
    /// Amount of offers which should occur before game ends
    pub(crate) const AMOUNT_OF_OFFERS: u32 = 100;
    /// Duration before key up should get registered
    pub(crate) const KEY_RELEASE_MILLIS: u32 = 500;
    /// Minimum duration offer should last
    pub(crate) const OFFER_DURATION_MIN_MILLIS: u32 = 2300;
    /// Maximum duration offer should last
    pub(crate) const OFFER_DURATION_MAX_MILLIS: u32 = 5400;

    /// Returns price range for the fruit in question
    pub(crate) fn range_for_fruit(fruit :&Fruit)->Range<u32>{
        match fruit{
            Fruit::Apples => {3..10}
            Fruit::Banana => {1..4}
            Fruit::Coconut => {3..7}
            Fruit::DragonFruit => {10..31}
            Fruit::Elderberry => {1..11}
        }
    }
}
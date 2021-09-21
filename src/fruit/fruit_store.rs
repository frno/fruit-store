use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{SystemTime, Duration};
use std::sync::{Arc, mpsc};
use rand::{thread_rng, Rng};

use crate::fruit::{Fruit, FruitOffer};
use crate::config::Config;

/// Fruit store will create a new Fruit offer when last offer has expired
/// creates a new thread for the Fruit store to live on.
/// Fruit store thread will exit when either
///     1. AMOUNT_OF_OFFERS defined in Config has occurred
///     2. game_running AtomicBool is false, signalling that the game should exit
pub(crate) struct FruitStore {
    keep_thread_alive: Arc<AtomicBool>
}

impl FruitStore {
    pub(crate) fn new()->FruitStore{
        return FruitStore { keep_thread_alive: Arc::new(AtomicBool::new(true)) };
    }

    pub(crate) fn create_fruit_thread(&mut self, game_running :Arc<AtomicBool>,next_turn_requested :Arc<AtomicBool>, tx : mpsc::Sender<FruitOffer>){
        let shared = Arc::clone(&self.keep_thread_alive);
        thread::spawn(move || {
            for offer_id in 0..Config::AMOUNT_OF_OFFERS {
                let fruit: Fruit = rand::random();
                let price= FruitStore::price_for_fruit(&fruit);

                let mut rng = thread_rng();
                let ms_offer_lasts = rng.gen_range(Config::OFFER_DURATION_MIN_MILLIS..Config::OFFER_DURATION_MAX_MILLIS);

                let offer = FruitOffer{
                    fruit,
                    price,
                    start: SystemTime::now(),
                    duration_ms: ms_offer_lasts,
                    id: offer_id
                };
                tx.send(offer).unwrap();

                // Instead of sleeping entire duration in one step, split into 10ms durations so if
                // player wishes to exit, then they get immediate response
                let mut ms_slept = 0;
                while ms_slept < ms_offer_lasts {
                    if !game_running.load(Ordering::Relaxed) {
                        break;
                    }
                    if next_turn_requested.load(Ordering::Relaxed) {
                        next_turn_requested.swap(false, Ordering::Relaxed);
                        break;
                    }
                    ms_slept = ms_slept + 10;
                    thread::sleep(Duration::from_millis(10));
                }

                if !game_running.load(Ordering::Relaxed) {
                    break;
                }
            }
            shared.swap(false, Ordering::Relaxed);
        });
    }

    pub(crate) fn price_for_fruit(fruit :&Fruit)->u32{
        let mut rng = thread_rng();
        rng.gen_range(Config::range_for_fruit(&fruit))
    }

    pub(crate) fn is_alive(&self) ->bool{
        if self.keep_thread_alive.load(Ordering::Relaxed) {
            return true;
        }
        return false;
    }
}

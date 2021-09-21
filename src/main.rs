mod console;
mod fruit;
mod player;
mod config;

use std::sync::{mpsc, Arc};
use std::{thread};
use std::sync::atomic::{AtomicBool, Ordering};
use std::ops::Range;
use std::time::Duration;

use crate::console::output::{Output};
use crate::console::input::{PlayerInteractionThreadHandler, PlayerInteractions};
use crate::console::key_handling::KeyHeldController;
use crate::player::Player;
use crate::fruit::fruit_store;
use crate::fruit::fruit_offer_controller::FruitOfferController;
use crate::config::Config;

fn main() {
    //Used to communicate towards fruit store thread if we wish for the game to be running
    let game_running: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    //Used to communicate towards fruit store thread if next turn is requested by player
    let next_turn_requested: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    //Setup up our console output and print intro
    let mut output = Output::new();
    output.print_intro();

    //Setup fruit store and mpsc channels used to retrieve new offers from the fruit store.
    //Offers time out, which is the reason for the fruit store running on separate thread.
    let (tx,rx) = mpsc::channel();
    let game_running_fruit_store = Arc::clone(&game_running);
    let next_turn_fruit_store = Arc::clone(&next_turn_requested);
    let mut fruit_store = fruit_store::FruitStore::new();
    fruit_store.create_fruit_thread(game_running_fruit_store, next_turn_fruit_store, tx);

    //Setup player interaction thread, so we can read key by key player's input
    let mut player_hid = PlayerInteractionThreadHandler::new();

    //Setup fruit offer controller, responsible for holding fruit store's current offer
    let mut offer_controller = FruitOfferController::new();

    let mut player = Player::new(Config::STARTING_CASH);

    //Key held controller does so if players keeps holding a key to repeatable buy they do not
    //mistakenly buy the next offer when the key held was started on the previous offer
    let mut key_held_controller = KeyHeldController::new();

    // loop
    //  Check fruit store is alive
    //  check for new offers,
    //      if new -> then assign fruit offer to offer_controller
    //      else -> {
    //          get current offer
    //          get player interaction (buy, sell, next offer, end)
    //          perform interaction
    //      }
    while fruit_store.is_alive(){
        let iter = rx.try_iter().next();
        match iter{
            None => {
                if offer_controller.has_offer() {
                    let offer = offer_controller.get_offer().unwrap();
                    let interaction_option = player_hid.get_player_interaction();
                    let elapsed = offer.millis_since_offer();

                    let proc = offer.get_duration_as_percent_of_elapsed(elapsed);

                    output.print_timeout(proc as u32);

                    let interaction = key_held_controller.filter_option_interaction(interaction_option, offer.get_id());
                    match interaction{
                        None =>
                            {
                                key_held_controller.cancel_if_elapsed();
                            }
                        Some(action) => {
                            match action{
                                PlayerInteractions::Buy => {
                                    let result = player.buy_offer(offer.get_fruit(),offer.get_price());
                                    match result{
                                        None => {
                                            output.print_no_offer();
                                        }
                                        Some(_) => {
                                            output.reset_player_feedback();
                                            output.print_player(&player);
                                        }
                                    }
                                }
                                PlayerInteractions::Sell => {
                                    let result = player.sell_offer(offer.get_fruit(),offer.get_price());
                                    match result{
                                        None => {
                                            output.print_no_such_in_inventory();
                                        }
                                        Some(_) => {
                                            output.reset_player_feedback();
                                            output.print_player(&player);
                                        }
                                    }
                                }
                                PlayerInteractions::Exit => {
                                    game_running.swap(false, Ordering::Relaxed);
                                }
                                PlayerInteractions::NextOffer => {
                                    output.print_skipping_turn();
                                    next_turn_requested.swap(true, Ordering::Relaxed);
                                }
                                PlayerInteractions::Info => {
                                    let range = Config::range_for_fruit(&offer.get_fruit());
                                    let print_range = Range{ start: range.start, end: range.end-1 };
                                    output.print_info(&offer.get_fruit(),print_range);
                                }
                            }
                        }
                    }
                }
                thread::sleep(Duration::from_millis(10))
            }
            Some(offer) => {
                &offer_controller.set_offer(&offer);
                let offer = &offer_controller.get_offer();
                match offer{
                    None => {}
                    Some(offer) => {
                        output.print_offer(&offer.get_fruit(), &offer.get_price(), &(Config::AMOUNT_OF_OFFERS - &offer.get_id() - 1));
                        output.print_player(&player);
                    }
                }
            }
        }
    }
    output.print_end(&player);
    player_hid.stop();
}




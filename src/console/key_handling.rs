use std::time::SystemTime;

use crate::console::input::PlayerInteractions;
use crate::config::Config;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct KeyHeld {
    last_interaction :PlayerInteractions,
    started: SystemTime,
    last: SystemTime,
    offer_id: u32
}

/// Keep track of the last interaction which the player performed and use this to determine if the
/// current interaction should be performed. This keeps the player from performing actions towards
/// newest offer by mistake when old offer expires.
pub(crate) struct KeyHeldController{
    current: Option<KeyHeld>
}

impl KeyHeldController{
    pub(crate) fn new() -> KeyHeldController {
        return KeyHeldController{ current: None };
    }

    /// Will release last performed action if key is not detected in duration determined by config
    pub(crate) fn cancel_if_elapsed(&mut self) -> bool {
        match &self.current{
            None => {}
            Some(c) => {
                if c.can_be_canceled() {
                    self.current = None;
                    return true;
                }
            }
        }
        return false;
    }

    /// Check if newest interaction should be performed. Returns None if last action was 'buy' or 'sell' and was started on a former offer
    pub(crate) fn filter_option_interaction(&mut self, interaction :Option<PlayerInteractions>, offer :u32) ->Option<PlayerInteractions> {
        return match interaction {
            None => { None }
            Some(val) => {
                self.filter_interaction(val, offer)
            }
        }
    }

    fn filter_interaction(&mut self, interaction :PlayerInteractions, offer :u32) ->Option<PlayerInteractions> {
        if let Some(ref mut current) = self.current{
            if current.last_interaction == interaction &&
                (interaction == PlayerInteractions::Buy ||
                    interaction == PlayerInteractions::Sell){
                current.update_last();
                if current.offer_id == offer {
                    return Some(interaction);
                }else{
                    //Have to wait for timeout here!
                    return None;
                }
            }
        }

        self.current = Some(KeyHeld{
            last_interaction: interaction.clone(),
            started: SystemTime::now(),
            last: SystemTime::now(),
            offer_id: offer
        });

        return Some(interaction);
    }
}

impl KeyHeld {
    /// update last time key was detected pressed
    fn update_last(&mut self){
        self.last = SystemTime::now();
    }

    /// check if key has expired and can be removed. The time key is held will vary between OS and configuration
    fn can_be_canceled(&self)->bool{
        let duration = self.last.elapsed();
        match duration{
            Ok(dur) => {
                dur.as_millis() > Config::KEY_RELEASE_MILLIS as u128
            }
            Err(_) => {
                false
            }
        }
    }
}
use termion::event::Key;
use termion::input::TermRead;

use std::io::{stdin};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;

/// All interactions player can perform
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum PlayerInteractions{
    Buy, //< Key: b, buy current offer
    Sell, //< Key: s, sell current offer
    NextOffer, //< Key: n, instantly go to next offer
    Exit, //< Key: e, end game and exit to prompt
    Info //< Key: i, print current fruit's price range
}

/// Player handler containing thread which takes console input from player.
pub(crate) struct PlayerInteractionThreadHandler {
    keyboard_thread_run :Arc<AtomicBool>,
    rx: Receiver<char>
}

impl PlayerInteractionThreadHandler {
    ///Start a new console thread to capture player input. Stop must be called for the thread to end.
    pub(crate) fn new()-> PlayerInteractionThreadHandler {
        let (tx,rx) = mpsc::channel();
        let keyboard_thread_run: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        keyboard_thread_run.swap(true,Ordering::Relaxed);
        let keyboard_thread_run_2 = Arc::clone(&keyboard_thread_run);
        read_thread(keyboard_thread_run_2, tx);
        return PlayerInteractionThreadHandler { keyboard_thread_run, rx };
    }
    /// Stops console thread by signalling exit
    pub(crate) fn stop(&mut self){
        self.keyboard_thread_run.swap(false,Ordering::Relaxed);
    }
    /// Consume last characters inputted by the user and output associated player interaction
    pub(crate) fn get_player_interaction(&self)->Option<PlayerInteractions>{
        let iter = self.rx.try_iter().next();
        match iter{
            None => {}
            Some(c) => {
                match c{
                    'b' => {
                        return Some(PlayerInteractions::Buy);
                    },
                    's' => {
                        return Some(PlayerInteractions::Sell);
                    },
                    'q' | 'e' | 'c' => {
                        return Some(PlayerInteractions::Exit);
                    }
                    'n' =>{
                        return Some(PlayerInteractions::NextOffer);
                    }
                    'i' =>{
                        return Some(PlayerInteractions::Info);
                    }
                    _ =>{}
                }
            }
        }
        return None;
    }
}

/// Create console thread to read keys and output read chars. Checks keyboard_thread_run to determine
/// when it is time to return.
fn read_thread(keyboard_thread_run :Arc<AtomicBool>, send_key : mpsc::Sender<char>) ->JoinHandle<()>{
    let stdin = stdin();
    return thread::spawn(move || {
        for c in stdin.keys() {
            if !keyboard_thread_run.load(Ordering::Relaxed) {
                break;
            }
            match c.unwrap() {
                Key::Char(c) =>
                    {
                        send_key.send(c).unwrap();
                        thread::sleep(Duration::from_millis(10));
                    },
                _ => {}
            }

            if !keyboard_thread_run.load(Ordering::Relaxed) {
                break;
            }
        }
    });
}

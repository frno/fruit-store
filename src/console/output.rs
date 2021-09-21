use std::ops::Range;
use std::io::{Stdout, stdout, Write};
use strum::IntoEnumIterator;
use termion::{style, color, clear};
use termion::raw::{IntoRawMode, RawTerminal};

use crate::fruit::Fruit;
use crate::config::Config;
use crate::{Player};

pub(crate) struct Output {
    stdout : RawTerminal<Stdout>,
    has_printed : bool,
    offers_left: String,
    offer_timeout: String,
    offer: String,
    status: String,
    player_feedback: String
}

impl Output {
    ///  Prints the game's intro screen with title and the different fruit price ranges
    pub(crate) fn print_intro(&mut self){
        self.println(format!("      ***********************"));
        self.println(format!("      *     {bold}{red}F{orange}R{yellow}UI{green}T S{cyan}TO{blue}R{purple}E{reset}     *",
                             bold  = style::Bold,
                             red = color::Fg(color::Red),
                             orange = color::Fg(color::Rgb(255,165,0)),
                             yellow = color::Fg(color::Yellow),
                             green = color::Fg(color::Green),
                             cyan = color::Fg(color::Rgb(0,255,255)),
                             blue = color::Fg(color::Blue),
                             purple = color::Fg(color::Rgb(128,0,128)),
                             reset = style::Reset));
        self.println(format!("      ***********************"));
        for fruit in Fruit::iter(){
            let range = Config::range_for_fruit(&fruit);
            let price_range = Range{ start: range.start, end: range.end-1 };
            let fruit_str = self.print_fruit(&fruit);
            self.println(format!("\t{price_range_start}$ to {price_range_end}$ {fruit}",
                    fruit = fruit_str,
                    price_range_start = price_range.start,
                    price_range_end = price_range.end));
        }
        self.println(String::from("      ***********************"));
        self.println(String::from(""));
    }

    /// Prints game over with the score being the amount of cash at the end of the game.
    /// no points are given for any inventory fruits, only stone cold cash
    pub(crate) fn print_end(&mut self, player: &Player) {
        let newline = format!("Game over, Your score is: {bold}{green}{cash}${reset}",
                              bold  = style::Bold,
                              cash = &player.get_cash(),
                              green = color::Fg(color::Green),
                              reset = style::Reset);
        self.println(newline);
    }

    /// Update first clears & moves the cursor up 5 lines. Then prints all 5 lines of information.
    /// All lines are cleared & all lines are printed for each update.
    fn update(&mut self){
        if self.has_printed {
            for _ in 0..5{
                self.print(format!("{reset_cursor_left}{clear}{move_cursor_up}",
                                   reset_cursor_left = termion::cursor::Left(100),
                                   move_cursor_up = termion::cursor::Up(1),
                                   clear = clear::CurrentLine));
            }
        }
        self.println(String::from(&self.offers_left));
        self.println(String::from(&self.offer_timeout));
        self.println(String::from(&self.offer));
        self.println(String::from(&self.status));
        self.println(String::from(&self.player_feedback));
        self.has_printed = true;
        self.stdout.flush().unwrap();
    }

    /// Create a new console session and put the terminal into raw mode
    pub(crate) fn new()-> Output {
        let stdout: RawTerminal<Stdout> = stdout().into_raw_mode().unwrap();
        Output { stdout, has_printed: false, offers_left: "".to_string(), offer_timeout: "".to_string(), offer: "".to_string(), status: "".to_string(), player_feedback: "".to_string() }
    }

    /// Will place debug string into player feedback line, before performing terminal update
    pub(crate) fn _print_debugln(&mut self, str:String){
        self.player_feedback = str;
        self.update();
    }

    /// Place 'Skipping turn' into player feedback line, before performing terminal update
    pub(crate) fn print_skipping_turn(&mut self){
        self.player_feedback = String::from("Skipping turn");
        self.update();
    }

    /// Place 'Not enough money' into player feedback line, before performing terminal update
    pub(crate) fn print_no_offer(&mut self)
    {
        self.player_feedback = String::from("Not enough money");
        self.update();
    }

    /// Place 'No such item in inventory' into player feedback line, before performing terminal update
    pub(crate) fn print_no_such_in_inventory(&mut self) {
        self.player_feedback = String::from("No such item in inventory");
        self.update();
    }

    /// Update current fruit offer and turns remaining lines, before performing terminal update
    pub(crate) fn print_offer(&mut self, fruit :&Fruit, price :&u32, offers_left:&u32){
        self.offers_left = format!("Offers left: {:0>2}", offers_left);

        self.offer =  format!("{fruit} for {price}$",
                              fruit=self.print_fruit(&fruit),
                              price=price);

        self.reset_player_feedback();

        self.print_timeout(100);
    }

    /// Set player feedback to default text which is a text showing player key options.
    /// Does not perform terminal update
    pub(crate) fn reset_player_feedback(&mut self){
        self.player_feedback = format!("[{green}{bold}b{reset}]uy [{red}{bold}s{reset}]ell [{blue}{bold}n{reset}]ext offer [{blue}{bold}e{reset}]nd game",
                                       bold = style::Bold,
                                       red = color::Fg(color::Red),
                                       green = color::Fg(color::Green),
                                       blue = color::Fg(color::Blue),
                                       reset = style::Reset);

    }

    /// Update line showing player's inventory of fruits, before performing terminal update
    pub(crate) fn print_player(&mut self, player :&Player){
        self.status = format!("{bold}{cash}{reset}$-{red}{bold}{apples}{reset}A-{yellow}{bold}{bananas}{reset}B-{brown}{bold}{coconuts}{reset}C-{magenta}{bold}{dragon_fruits}{reset}D-{green}{bold}{elder_berries}{reset}E",
                              cash =player.get_cash(),
                              bold = style::Bold,
                              red = color::Fg(color::Red),
                              yellow = color::Fg(color::Yellow),
                              brown = color::Fg(color::Rgb(210,105,30)),
                              magenta = color::Fg(color::Magenta),
                              green = color::Fg(color::Green),
                              apples = player.get_amount_of_fruit(Fruit::Apples),
                              bananas = player.get_amount_of_fruit(Fruit::Banana),
                              coconuts = player.get_amount_of_fruit(Fruit::Coconut),
                              dragon_fruits = player.get_amount_of_fruit(Fruit::DragonFruit),
                              elder_berries = player.get_amount_of_fruit(Fruit::Elderberry),
                              reset = style::Reset);
        self.update();
    }

    /// Set fruit's price range info into player feedback line, before performing terminal update
    pub(crate) fn print_info(&mut self, fruit :&Fruit, price_range :Range<u32>){
        self.player_feedback = format!("{fruit} range is [{price_range_start} to {price_range_end}]",
                                       fruit = self.print_fruit(&fruit),
                                       price_range_start = price_range.start,
                                       price_range_end = price_range.end);

        self.update();
    }

    /// Update offer timeout line which is a line consisting of max 20 '_' above the current offer
    /// the amount of '_' is between 0 and 20. Color of line starts green but becomes red when
    /// little time is left. Performs terminal update if there is need for it.
    pub(crate) fn print_timeout(&mut self, proc:u32){
        let mut amount = proc / 5;

        if amount > 20 { amount = 20 };

        let mut progress;
        if amount > 13 {
            progress = format!("{green}",
                      green = color::Fg(color::Green));
        }else if amount > 7 {
            progress = format!("{yellow}",
                               yellow = color::Fg(color::Yellow));
        }else {
            progress = format!("{red}",
                               red = color::Fg(color::Red));
        }

        for _ in 1..amount+1{
            progress = format!("{}{}", progress, "_");
        }
        progress = format!("{}{reset}", progress, reset = style::Reset);

        if self.offer_timeout != progress {
            self.offer_timeout = progress;
            self.update();
        }
    }

    /// Returns formatted string for fruit name
    fn print_fruit(&mut self, fruit :&Fruit)-> String{
        match fruit{
            Fruit::Apples => {
                return format!("{red}Apple{reset}",
                                   red = color::Fg(color::Red),
                                   reset = style::Reset);
            }
            Fruit::Banana => {
                return format!("{yellow}Banana{reset}",
                                   yellow = color::Fg(color::Yellow),
                                   reset = style::Reset);
            }
            Fruit::Coconut => {
                return format!("{brown}Coconut{reset}",
                                   brown = color::Fg(color::Rgb(210,105,30)),
                                   reset = style::Reset);
            }
            Fruit::DragonFruit => {
                return format!("{magenta}DragonFruit{reset}",
                                   magenta = color::Fg(color::Magenta),
                                   reset = style::Reset);
            }
            Fruit::Elderberry => {
                return format!("{green}Elderberry{reset}",
                                   green = color::Fg(color::Green),
                                   reset = style::Reset);
            }
        }
    }

    /// Writes to terminal with '\n\r' ending
    fn println(&mut self, str: String) {
        write!(self.stdout,"{}\n\r", str).unwrap();
    }

    /// Writes to terminal
    fn print(&mut self, str: String)
    {
        write!(self.stdout,"{}", str).unwrap();
    }
}
extern crate clap;
use clap::{Arg, App, SubCommand};

extern crate rand;
mod Card;
mod Deck;
use Deck::create_deck;

#[macro_use]
extern crate serde_derive;

fn main() {
    //create deck
    //let deck = create_deck(num_cards: 10, exp_to_grant = 2000, deck_name: "Deck_1");
    //
    let deck = create_deck(10, 2000,"this_is_a_deck".to_owned());



    
    //print deck
    
    //Save deck

    //Load deck


}

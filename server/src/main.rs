extern crate rand;
extern crate serde_json;
extern crate serde_yaml;
extern crate cardgame_board;

mod gameloop;
mod action;

use cardgame_board::*;
use gameloop::gameloop;
//use std::io::{self, BufRead};

fn main() {
    //Read in the decks
    let p1_deck = read_deck_from_file("p1.deck".to_owned());
    let p2_deck = read_deck_from_file("p2.deck".to_owned());


    //For now we can just clone the deck if it doesnt load.
    //later that should cause you to pick a different deck
    let base_deck = Deck::default();
    let mut p2 = create_player("p2".to_owned(), base_deck.clone());
    let mut p1 = create_player("p1".to_owned(), base_deck.clone());

    //let mut p1_deck = p2_deck.clone();
    if p1_deck.is_ok() {
        p1.deck = p1_deck.unwrap();
    } else {
        p1.deck = create_deck(30, 6000, "p1.deck".to_owned());
    }
    if p2_deck.is_ok() {
        p2.deck = p2_deck.unwrap();
    } else {
        p2.deck = create_deck(30, 6000, "p2.deck".to_owned());
    }



    gameloop(p1, p2);
}

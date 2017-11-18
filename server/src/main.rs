extern crate rand; extern crate serde_json;
extern crate serde_yaml;
extern crate cardgame_board;
//#[macro_use] 
extern crate serde_derive;

mod gameloop;
mod action;

use cardgame_board::*;
use gameloop::gameloop;
use std::sync::mpsc::channel;
use std::thread;
use std::io::{self, BufRead};

fn main() {

    //Send data back and forth for players
    let (sender_p1, receiver_p1) = channel();
    let (sender_p1_home, receiver_p1_home) = channel();
    let (sender_p2, receiver_p2) = channel();
    let (sender_p2_home, receiver_p2_home) = channel();

    thread::spawn(move || {
        let p1_deck = read_deck_from_file("p1.deck".to_owned());
        let p2_deck = read_deck_from_file("p2.deck".to_owned());

        let base_deck = Deck::default();
        //For now we can just clone the deck if it doesnt load.
        //later that should cause you to pick a different deck
        let mut p1 = create_player("p1".to_owned(), base_deck.clone());
        let mut p2 = create_player("p2".to_owned(), base_deck.clone());

        //let mut p1_deck = p2_deck.clone();
        if p1_deck.is_ok() { 
            p1.deck = p1_deck.unwrap();
        }
        else {
            p1.deck = create_deck(30, 6000, "p1.deck".to_owned());
        }
        if p2_deck.is_ok() { 
            p2.deck = p2_deck.unwrap();
        }
        else {
            p2.deck = create_deck(30, 6000, "p2.deck".to_owned());
        }

        gameloop(p1, p2, sender_p1_home, receiver_p1, sender_p2_home, receiver_p2);
    });



    //Player 1 
    //thread::spawn(move || {
        //Wait untill the signal
        let mut turn_going = true;
        while turn_going {
            //Wait till they ask for data
            let ask = receiver_p1_home.recv();
            let ask_str = ask.unwrap();
            if ask_str == "end" { 
                turn_going = false;
            }
            else if ask_str == "ready" {
                //They said something, for now we will print it out
                println!("From the gameloop to p1: {}", ask_str);

                //Read in data, post it
                let stdin = io::stdin();
                let mut line = String::new();
                stdin.lock().read_line(&mut line).expect("could not read line");
                sender_p1.send(line.trim_right_matches("\r\n").to_string());
            }
            else { 
                //Deal with incomming data
                println!("Getting data");
                println!("{}", ask_str);

            }
        }

    //});

}


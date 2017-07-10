extern crate rand;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate clap;

mod board;
mod client;
mod action;

use clap::{Arg, App, SubCommand};
use board::*;
use client::gameloop;

fn main() {
    //clap options
    let matches = App::new("Cardgame")
        //.version(crate_version!())
        .author("Kyle Merfeld. <kmerfeld1@gmail.com>")
        .about("Tries to let you play a game")
        .subcommand(SubCommand::with_name("create")
                    .version(crate_version!())
                    .about("Create deck")
                    .arg(Arg::with_name("deck_name")
                         .long("name")
                         .short("n")
                         .help("The name of the deck")
                         .takes_value(true)
                         .required(true))
                    .arg(Arg::with_name("exp")
                         .long("exp")
                         .short("e")
                         .help("How much exp to grant to your cards")
                         .takes_value(true)
                         .default_value("2000"))
                    .arg(Arg::with_name("number_of_cards")
                         .long("cards")
                         .short("c")
                         .help("The number of cards in the deck")
                         .takes_value(true)
                         .default_value("30")))
        .subcommand(SubCommand::with_name("client")
                    .arg(Arg::with_name("test")
                        .long("start")
                        .short("s")
                        .takes_value(true)
                        .default_value("thing")))
        .subcommand(SubCommand::with_name("server")
                    .about("Start a client (not working yet)"))
        .get_matches();


    //handle the options provided
    match matches.subcommand() {
        //When the create subcommand is run
        ("create", Some(create_matches)) =>{
            // Now we have a reference to create's matches
            println!("Creating a deck with {} cards", create_matches.value_of("number_of_cards").unwrap());
            println!("Creating a deck with {} exp ", create_matches.value_of("exp").unwrap());
            println!("Creating a deck named {}", create_matches.value_of("deck_name").unwrap());
            //Create a deck with options
            create_deck(create_matches.value_of("number_of_cards").unwrap().parse::<i32>().unwrap(),
            create_matches.value_of("exp").unwrap().parse::<i32>().unwrap(),
            create_matches.value_of("deck_name").unwrap().to_owned());
        },
        /*
        ("server", Some(server_matches)) => {
            println!("Not implemented yet");
        },
        */
        ("client", Some(client_matches)) => {
    
            //make 2 players with decks
            let mut p1_deck = Deck::read_deck_from_file("p1.deck".to_owned());
            let p2_deck = create_deck(10, 2000, "p2.deck".to_owned());

            //let mut p1_deck = p2_deck.clone();
            if p1_deck.is_ok() { 

                let p1 = create_player("p1".to_owned(), p1_deck.ok().unwrap());
                let p2 = create_player("p2".to_owned(), p2_deck);
                gameloop(p1, p2);

            }
                    },
        ("", None)   => println!("Look at ./cardgame --help"),
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}

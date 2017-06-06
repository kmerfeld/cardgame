extern crate clap;
use clap::{Arg, App, SubCommand};

extern crate rand;
mod Card;
mod Deck;
use Deck::create_deck;

#[macro_use]
extern crate serde_derive;



fn main() {
    let matches = App::new("Cardgame")
        //.version(crate_version!())
        .author("Kyle Merfeld. <kmerfeld1@gmail.com>")
        .about("Tries to let you play a game")
        .subcommand(SubCommand::with_name("create")
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
                         .required(true))
                    .arg(Arg::with_name("number_of_cards")
                         .long("cards")
                         .short("c")
                         .help("The number of cards in the deck")
                         .takes_value(true)
                         .required(true)))
        .subcommand(SubCommand::with_name("server")
                    .about("Start a server (not working yet)"))
        .subcommand(SubCommand::with_name("client")
                    .about("Start a client (not working yet)"))
        .get_matches();



    match matches.subcommand() {
        ("create", Some(create_matches)) =>{
            // Now we have a reference to create's matches
            println!("Creating a deck with {} cards", create_matches.value_of("number_of_cards").unwrap());
            println!("Creating a deck with {} exp ", create_matches.value_of("exp").unwrap());
            println!("Creating a deck named {}", create_matches.value_of("deck_name").unwrap());
            create_deck(create_matches.value_of("number_of_cards").unwrap().parse::<i32>().unwrap(),
            create_matches.value_of("exp").unwrap().parse::<i32>().unwrap(),
            create_matches.value_of("deck_name").unwrap().to_owned());
        },
        ("server", Some(server_matches)) => {
            println!("Not implemented yet");
        },
        ("client", Some(server_matches)) => {
            println!("Not implemented yet");
        },
        ("", None)   => println!("Look at ./cardgame --help"),
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}

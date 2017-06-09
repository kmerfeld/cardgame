extern crate serde;
extern crate serde_json;
extern crate rand;

use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
//use std::error::Error;
//use std::path::Path;


use card::*;
//use card::Ability;
use card::CardClass;

#[derive(Serialize, Deserialize)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub name_of_deck: String,
}

impl Deck {
    pub fn new(&self, deck_name: String) -> Deck{
        let deck = Deck { cards: Vec::new(), name_of_deck: deck_name};
        return deck;
    }

    pub fn print(&self) {
        for i in &self.cards {
            i.pretty_print();
            println!("");

        }
    }
    pub fn save_to_file(&self) {
        let mut file = File::create(&self.name_of_deck).unwrap();
        for i in &self.cards {
            let c = Card {
                name: i.name.to_owned(),
                card_class: i.card_class.clone(),
                health: i.health.to_owned(),
                attack: i.attack.to_owned(),
                level:i.level.to_owned(),
                exp: i.exp.to_owned(),
                durability: i.durability.to_owned(),
                abilities: i.abilities.to_owned(),
                cost: i.cost,
            };
            let j = serde_json::to_string(&c).unwrap();
            file.write_all(j.as_bytes()).unwrap();
        }
    }
}

//TODO: this needs to be fixed
/*
pub fn read_deck_from_file<P: AsRef<Path>>(path: P) -> Result<Deck, Box<Error>> {
    let file = File::open(path)?;
    let u = serde_json::from_reader(file)?;
    Ok(u)
}
*/

pub fn create_deck(num_cards: i32, mut exp_to_grant: i32, deck_name: String) -> Deck{
    //Generate up some cards
    let mut card_vec = Vec::new();


    //read in classes
    for _ in 0..num_cards {

        let abi = Vec::new();
        let z = CardClass{ name:"test".to_owned(), ability_list:abi };
        let x = Card {
            name: "test".to_owned(),
            health: rand::thread_rng().gen_range(1, 10),
            attack: rand::thread_rng().gen_range(1, 10),
            level: 1,
            exp: 0,
            durability: 10,
            card_class: z,
            abilities: Vec::new(),
            cost: 1,
        };
        card_vec.push(x)
    }

    let mut deck = Deck { cards: card_vec, name_of_deck: deck_name};

    // Grant cards a few levels 
    // This part will grant levels 1-5 to some of your cards untill you
    // run out of experience to give
    while exp_to_grant > 0 {
        //Determine which card gets booseted
        let mut which_card = 1;
        if num_cards > 1 {
            which_card = rand::thread_rng().gen_range(1, deck.cards.len());
        }
        //figure out how many levels to give it
        let mut to_give = 0;
        if exp_to_grant > 600 {
            match rand::thread_rng().gen_range(1,5) {
                1 => to_give = 100,
                2 => to_give = 225,
                3 => to_give = 350,
                4 => to_give = 475,
                5 => to_give = 600,
                _ => {},
            };
            deck.cards[which_card].give_exp(to_give)
        }
        else {
            to_give = exp_to_grant;
        }
        exp_to_grant = exp_to_grant - to_give;
    }


    deck.print();
    deck.save_to_file();

    return deck;

}


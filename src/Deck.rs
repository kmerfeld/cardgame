extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use Card::Card;

#[derive(Serialize, Deserialize)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn print(&self) {
        for i in &self.cards {
            i.pretty_print();
            println!("");

        }
    }
    pub fn save_to_file(&self) {
        let deck_name = "this".to_owned();
        let mut file = File::create(deck_name).unwrap();
        for i in &self.cards {
            let c = Card {
                name: i.name.to_owned(),
                class: i.class.to_owned(),
                health: i.health.to_owned(),
                attack: i.attack.to_owned(),
                level:i.level.to_owned(),
                exp: i.exp.to_owned(),
                durability: i.durability.to_owned(),
                abilities: i.abilities.to_owned(),
            };
            let j = serde_json::to_string(&c).unwrap();
            println!("{:?}",j);
            file.write_all(j.as_bytes()).unwrap();
        }
    }
}

extern crate serde;
extern crate serde_json;
extern crate rand;

use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::fmt;




// The field where you play on
pub struct Board {
    // The players
    pub player_1: Player,
    pub player_2: Player,
}

impl Board {
    pub fn new(p1_name: String, p2_name: String, p1_deck: Deck, p2_deck: Deck) -> Board {
        let p1 = create_player(p1_name, p1_deck);
        let p2 = create_player(p2_name, p2_deck);
        let board = Board{ 
            player_1: p1, 
            player_2: p2,
        };


        return board;
    }

    //Print out the board state to STDOUT
    pub fn print(&self) {


        println!("Player 2 is: {}", &self.player_2.name);
        println!("In his hand is the following:");
        for i in &self.player_2.hand {
            i.pretty_print();
        }
        println!("on his board is the following:");
        for i in &self.player_2.field {
            i.pretty_print();
        }
    }
}

//Allow something to draw a card
pub fn draw_card<'a>(player: &'a mut Player) {
    if player.deck.cards.len() < 1 {}
    else {
        let topcard: Card = player.deck.cards.pop().unwrap();
        println!("{}", topcard);
        player.hand.push(topcard);

    }


}
// The players who interact with the board
#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub deck: Deck,
    pub field: Vec<Card>,
    pub hand: Vec<Card>,

}
impl Player {
    pub fn print(&self) {
        println!("Player is: {}", &self.name);
        println!("In his hand is the following:");
        for i in &self.hand {
            i.pretty_print();
        }
        println!("on his board is the following:");
        for i in &self.field {
            i.pretty_print();
        }
    }
}

pub fn create_player(name: String, deck: Deck) -> Player {
    let hand = Vec::new();
    let field = Vec::new();
    let player = Player { name: name, deck: deck, hand: hand, field: field};
    return player;
}

// The actions that the players do
// These go "player" does "action" to "target".
// The board then decides what happens
pub struct Event {
    caster: Player,
    action: String, //This will be an action
    target: Card,
}


/* Start deck section */
#[derive(Clone, Serialize, Deserialize)]
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


    deck.save_to_file();

    return deck;

}

/* Start Card section */
#[derive(Clone, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub abilities: Vec<String>,
    pub health: i32,
    pub attack: i32,
    pub level: i32,
    pub exp: i32,
    pub durability: i32,
    pub card_class: CardClass,
    pub cost:i32,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.health, self.attack)
    }
}
impl Card {
    //Grant exp to a card
    pub fn give_exp(&mut self, x: i32) {
        self.exp = &self.exp + x;
        if self.exp > (self.level * 125 + 100 ) {
            self.level = &self.level + 1;
            // Here we roll on the table based on the cards class
        }
    }


    pub fn pretty_print(&self) {
        println!("name:\t{}|class:\t{}|health:\t{}|attack:\t{}|level:\t{}|exp:\t{}|dura:\t{}",
                 &self.name,
                 &self.card_class,
                 &self.health,
                 &self.attack,
                 &self.level,
                 &self.exp,
                 &self.durability);
        for i in &self.abilities {
            println!("{}", i);

        }
    }
}

#[derive(Clone,Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
    pub level_requirement: i32,
    pub target: String,
    pub effect: String,
}

impl fmt::Display for Ability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.name, self.level_requirement, self.target, self.effect)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CardClass {
    pub name: String,
    pub ability_list: Vec<Ability>,
}

impl fmt::Display for CardClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}



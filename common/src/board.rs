extern crate serde;
extern crate serde_json;
extern crate rand;

use std::error::Error;
use std::path::Path;

use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::fmt;


/* The Card section */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub abilities: Vec<Ability>,
    pub health: i32,
    pub attack: i32,
    pub level: i32,
    pub exp: i32,
    pub durability: i32,
    pub card_class: CardClass,
    pub cost:i32,
    pub id:i32,
    pub fatigued: bool,
}

impl Default for Card {
    fn default()  -> Card{
        return Card{
            name: "Default".to_string(),
            id: 0,
            health: 0,
            attack: 0,
            level: 0,
            exp: 0,
            durability: 0,
            card_class: CardClass::default(),
            abilities: Vec::new(),
            cost: 0,
            fatigued: true,
        };
    }
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
        println!("[name:{}] | [class:{}] | [id:{}] | [health:{}] | [attack:{}] | [level:{}] | [exp:{}] | [cost:{}] | [fatigued:{}] | [dura:{}]",
                 &self.name,
                 &self.card_class,
                 &self.id,
                 &self.health,
                 &self.attack,
                 &self.level,
                 &self.exp,
                 &self.cost,
                 &self.fatigued,
                 &self.durability);

        for i in &self.abilities {
            println!("{}", i.all_pick);
            for j in &i.ability_raws {
                println!("{}", j);

            }
        }
    }
}



#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct AbilityRaw {
    pub name: String,
    pub level_requirement: i32,
    pub target: String,
    pub effect: String,
    pub trigger: String,
}
impl Default for AbilityRaw{
    fn default() -> AbilityRaw {
        return AbilityRaw{
            name: "Default".to_owned(),
            level_requirement: 0,
            target: "".to_owned(),
            effect: "".to_owned(),
            trigger: "".to_owned(),
        };
    }
}


impl fmt::Display for AbilityRaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.name, self.level_requirement, self.target, self.effect)
    }
}

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Ability {
    pub all_pick : String,
    pub ability_raws: Vec<AbilityRaw>,
}
impl Default for Ability {
    fn default() -> Ability {
        return Ability{ all_pick: "all".to_owned(), ability_raws: Vec::new()};

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardClass {
    pub name: String,
    pub ability_list: Vec<Ability>,
}
impl Default for CardClass {
    fn default() -> CardClass {
        return CardClass{ name: "Default".to_string(), ability_list: Vec::new()};
    }
}

impl fmt::Display for CardClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/*  The player section */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub deck: Deck,
    pub field: Vec<Card>,
    pub hand: Vec<Card>,
    pub graveyard: Vec<Card>,
    pub health: i32,
    pub id: i32,

}
impl Player {
    pub fn print(&self) {
        println!("Player is: {}", &self.name);
        println!("They have {} health", &self.health);
        println!("In the hand is the following:");
        for i in &self.hand {
            i.pretty_print();
        }
        println!("\nOn the  board is the following:");
        for i in &self.field {
            i.pretty_print();
        }
        println!("\nIn the graveyard is the following:");
        for i in &self.graveyard {
            i.pretty_print();
        }
    }
}

pub fn create_player(name: String, deck: Deck) -> Player {
    let hand = Vec::new();
    let field = Vec::new();
    let graveyard = Vec::new();
    let health: i32 = 30;
    let player = Player { name: name, id: 0, health: health, deck: deck, hand: hand, field: field, graveyard: graveyard};
    return player;
}

// The actions that the players do
// These go "player" does "action" to "target".
// The board then decides what happens
pub struct Event {
    caster: Player, //ex: "player1"
    action: String, //ex: "Destroy"
    target: i32,   //ex: "<target id>
    text: String,   //ex: "Destroy target entity
}


/* The deck section */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub name_of_deck: String,
    pub cards: Vec<Card>,

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
        let j = serde_json::to_string(&self).unwrap();
        file.write_all(j.as_bytes()).unwrap();
    }
    pub fn read_deck_from_file<P: AsRef<Path>>(path: P) -> Result<Deck, Box<Error>> {
        // Open the file in read-only mode.
        let file = File::open(path)?;

        let u: Deck = serde_json::from_reader(file)?;

        // Return the `User`.
        Ok(u)
    }

}


fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<String, Box<Error>> {
    // Open the file in read-only mode.
    let file = File::open(path)?;

    let u = serde_json::from_reader(file)?;

    // Return the `User`.
    Ok(u)
}

//TODO: investigate this with 0 cards and 0 exp
pub fn create_deck(num_cards: i32, mut exp_to_grant: i32, deck_name: String) -> Deck{
    //Generate up some cards
    let mut card_vec = Vec::new();

    //let json_data = read_json_from_file("abilities").unwrap();
    //let a = serde_json::from_str(&json_data).unwrap();
    //let spell = a[Spellcaster];
    //read in classes
    for _ in 0..num_cards {

        let abi = Vec::new();
        let z = CardClass{ name:"test".to_owned(), ability_list:abi.clone() };


        let mut tmp_name = "Default".to_owned();
        let mut tmp_health = 0;
        let mut tmp_attack = 0;
        let mut tmp_abilities: Vec<Ability> = Vec::new();
        let mut tmp_card_class = z;


        let class = rand::thread_rng().gen_range(1, 4);
        //Spellcaster
        if class == 1 {
            //Get an ability
            tmp_name = "Level 1 Spellcaster".to_owned();
            tmp_health = rand::thread_rng().gen_range(1, 2);
            tmp_attack = rand::thread_rng().gen_range(0, 1);
            tmp_card_class = CardClass{ name: "Spellcaster".to_owned(), ability_list:abi.clone() };

            let mut a: Ability = Ability::default();
            a.ability_raws.push(AbilityRaw{
                name:"Death bolt".to_owned(),
                level_requirement:0,
                target:"target enemy creature".to_owned(),
                trigger: "on_play".to_owned(),
                effect:"destroy".to_owned()
            });
            tmp_abilities.push(a);


        }
        //Attacker
        if class == 2 {
            tmp_name = "Level 1 Attacker".to_owned();
            tmp_health = rand::thread_rng().gen_range(1, 3);
            tmp_attack = rand::thread_rng().gen_range(2, 6);
            tmp_card_class = CardClass{ name: "Attacker".to_owned(), ability_list:abi.clone() };

            let mut a: Ability = Ability::default();
            a.ability_raws.push(AbilityRaw{
                name:"rag party".to_owned(),
                level_requirement:0,
                target:"both_fields".to_owned(),
                trigger: "on_play".to_owned(),
                effect:"modify attack 5".to_owned()
            });
            tmp_abilities.push(a);


        }
        //Defender
        if class == 3 {
            tmp_name = "Level 1 Defender".to_owned();
            tmp_health = rand::thread_rng().gen_range(2, 5);
            tmp_attack = rand::thread_rng().gen_range(1, 2);
            tmp_card_class = CardClass{ name: "Defender".to_owned(), ability_list:abi.clone() };
            let mut a: Ability = Ability::default();
            a.ability_raws.push(AbilityRaw{
                name:"Block".to_owned(),
                level_requirement:0,
                target:"none".to_owned(),
                trigger: "on_player_attack".to_owned(),
                effect:"enemy cant attack hero".to_owned()
            });
            tmp_abilities.push(a);

        }
        let x = Card {
            name: tmp_name,
            id: 0,
            health: tmp_health,
            attack: tmp_attack,
            level: 0,
            exp: 0,
            durability: 10,
            card_class: tmp_card_class,
            abilities: tmp_abilities,
            cost: 1,
            fatigued: true,
        };
        card_vec.push(x);
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

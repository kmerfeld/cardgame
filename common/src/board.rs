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
    pub class_name: String,
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
            class_name: "Default".to_owned(),
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
                 &self.class_name,
                 &self.id,
                 &self.health,
                 &self.attack,
                 &self.level,
                 &self.exp,
                 &self.cost,
                 &self.fatigued,
                 &self.durability);

        for i in &self.abilities {
            println!("Name: {} Trigger: {}", i.name, i.trigger);
            for j in &i.ability_raws {
                println!("{}", j);

            }
        }
    }
}



#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct AbilityRaw {

    pub target: String,
    pub effect: String,

}
impl Default for AbilityRaw{
    fn default() -> AbilityRaw {
        return AbilityRaw{
            target: "".to_owned(),
            effect: "".to_owned(),
        };
    }
}


impl fmt::Display for AbilityRaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.target, self.effect)
    }
}

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
    pub level_requirement: i32,
    pub all_pick : String,
    pub ability_raws: Vec<AbilityRaw>,
    pub trigger: String,
}
impl Default for Ability {
    fn default() -> Ability {
        return Ability{ name: "ability_1".to_string(), level_requirement: 0, all_pick: "all".to_owned(), ability_raws: Vec::new(), trigger: "on_play".to_owned()};

    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardClass {
    pub name: String,
    pub ability_list: Vec<Ability>,
    pub init_health: i32,
    pub init_attack: i32,
    pub init_stats: Vec<Vec<i32>>,
    pub init_points: Vec<Vec<i32>>,
    pub level_stats: Vec<Vec<i32>>,
    pub level_points: Vec<Vec<i32>>,
}


impl Default for CardClass {
    fn default() -> CardClass {
        return CardClass{ 
            name: "Default".to_string(),
            ability_list: Vec::new(),
            init_health: 0,
            init_attack: 0,
            init_stats: vec![vec![0]],
            init_points: vec![vec![0]],
            level_stats: vec![vec![0]],
            level_points: vec![vec![0]],
        };
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
impl Default for Player {
    fn default() -> Player {
        return Player{ name: "default".to_owned(), deck: Deck::default(), field: Vec::new(), hand: Vec::new(), graveyard: Vec::new(), health: 0, id: 0 };    
    }
}
impl Player {
    /*
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
    */
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
/*
   pub struct Event {
   caster: Player, //ex: "player1"
   action: String, //ex: "Destroy"
   target: i32,   //ex: "<target id>
   text: String,   //ex: "Destroy target entity
   }
   */

/* The deck section */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub name_of_deck: String,
    pub cards: Vec<Card>,
    pub card_classes: Vec<CardClass>,

}

impl Default for Deck {
    fn default()  -> Deck{
        return Deck{
            name_of_deck: "default".to_owned(),
            cards: Vec::new(),
            card_classes: Vec::new()};
    }
}

impl Deck {
    pub fn print(&self) {
        println!("Name: {}", &self.name_of_deck);
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
}

//TODO: use generic to fuse these two functions
pub fn read_deck_from_file<P: AsRef<Path>>(path: P) -> Result<Deck, Box<Error>> {
    let file = File::open(path)?;
    let u: Deck = serde_json::from_reader(file)?;
    Ok(u)
}


fn read_card_class<P: AsRef<Path>>(path: P) -> Result<Vec<CardClass>, Box<Error>> {
    let file = File::open(path)?;
    let u = serde_json::from_reader(file)?;
    Ok(u)
}


pub fn create_card<'a>(deck: &'a Deck) -> Card {

    //Pick a random class
    let class_i = rand::thread_rng().gen_range(0, deck.card_classes.len());

    let class: &CardClass = &deck.card_classes[class_i];
    let mut card = Card::default();

    //card.name = format!("Level 0 {}", class.name); 
    card.health = class.init_health;
    card.attack = class.init_attack;
    card.level = 0;
    card.durability = 10;
    card.class_name = class.name.clone();

    let mut stat_points: i32= 0;
    //get mana cost
    //What this is doing is looking at a 2d vec to try and figure what mana 
    //cost to give to the card
    //
    //it uses class.init_points which can look something like this
    //mana cost,	Chance %,	Stat Points
    //1	25	2
    //2	40	2
    //3	20	1
    //4	9	2
    //5	5	2
    //6	0	1
    //7	0	11
    //8	0	12
    //9	0	13
    //10	1	15
    //
    // Once it gets a random int 1-100 it will try the first one, in this case 25%.
    // if the number is 1-25% it will be a card with mana cost 1,
    // if it isnt in that range it will at 25 to "last" and see if the next one is 
    // in between last and the next range + last

    //Get card mana cost
    let mut last: i32 = 1;
    let value = rand::thread_rng().gen_range(1, 100);
    for i in 0..class.init_points.len()  {
        if value < class.init_stats[i][1] + last -1|| value == class.init_stats[i][1] + last -1{
            card.cost = (i + 1) as i32;
            break;
        }
        else {
            last += class.init_stats[i][1];
        }
    }

    if card.cost == 0 {
        println!("your init_stats values are off. total is > 100, setting card to 1 mana cost");
        card.cost = 1;
    }
    //Use up points
    //Get the total amount of points we will award
    println!("card cost = {}", card.cost);


    //For each level
    for i in 0..card.cost {
        //for each stat point gained that level
        for _ in 0..class.init_stats[i as usize][2] {
            let value = rand::thread_rng().gen_range(0, 100);
            //Here we will figure out which we want

            let points = class.init_points[i as usize].clone();

            println!("value:{}, ability:{}, attack:{}, health:{}", value, points[1], points[2], points[3]);
            //add ability
            if value < points[1] {
                add_ability(&mut card, &class);
                println!("adding an ability");
            }
            //add attack
            else if value < points[1] + points[2]  {
                card.attack += 1;
                println!("adding an attack point");
            }
            else if value < points[1] + points[2] + points[3] {
                card.health += 1;
                println!("adding a health point");
            }
        }
    }

    println!("");

    return card;
}

fn add_ability<'a>(card: &'a mut Card, class: &'a CardClass) {
    //we check a maximum of all the cards
    println!("getting an ability");

    for _ in 0..class.ability_list.len() {
        //pick a random ability
        let value = rand::thread_rng().choose(&class.ability_list).unwrap().clone();
        //if level_requirement doesnt disqualify the ability
        if value.level_requirement < card.level + 1 {
            card.abilities.push(value);
            break;
        }
    }
}


//TODO: investigate this with 0 cards and 0 exp
pub fn create_deck(num_cards: i32, mut exp_to_grant: i32, deck_name: String) -> Deck{
    //Generate up some cards
    let mut card_vec: Vec<Card> = Vec::new();



    //Check if file exists
    println!("Does the abilities file exists?");
    println!("{}", Path::new("abilities.json").exists());

    let input = read_card_class("abilities.json".to_owned());

    //Check that it seems good
    if input.is_ok() {
        println!("input is good");
    }


    else {
        println!("Could not interpret abilities file");

    }

    let classes: Vec<CardClass> = input.unwrap();
    let mut deck = Deck{ name_of_deck: "thing".to_owned(), card_classes: classes.clone(), ..Deck::default()};
    for _ in 0..num_cards {
        let which_class = rand::thread_rng().gen_range(0, classes.len());
        println!("Creating a card of type {}", classes[which_class].name);
        card_vec.push(create_card(&deck));
    }
    deck.cards = card_vec;

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

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate rand;

use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

use std::error::Error;
use std::path::Path;

use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::fmt;

///The Board is the structure that Holds the two players, All their objects,
///all effects that apply to everything in both fields, and all the events that have taken place
pub struct Board {
    pub next_entity: i32,
    pub player_1: Player,
    pub player_2: Player,
    pub events: Vec<Event>,
}

impl Default for Board {
    fn default() -> Board {
        return Board {
            next_entity: 0,
            player_1: Player::default(),
            player_2: Player::default(),
            events: Vec::new(),
        };
    }
}

impl Board {
    pub fn add_player(&mut self, player: Player) -> Result<String, String> {
        //Check if p1 is empty
        if self.player_1.name == "Default" {
            self.player_1 = player;
            Ok("Player added".to_owned())
        }
        else if self.player_2.name == "Default" {
            if self.player_1.name == player.name {
                Err("The other player already has this name".to_owned())
            }
            else {
                self.player_2 = player;
                Ok("Player added".to_owned())
            }
        } else {
            Err("Both player spots are already filled.".to_owned())
        }
    }
}
///Every action in the game will be of the event struct.
pub struct Event {
    pub from_player: i32,
    pub visibility: i32,
    pub action: String,
    pub action_param:  Vec<String>,

}

/* The Card section */
///Cards the units a player collects, and uses for battles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub abilities: Vec<Ability>,
    pub tmp_abilities: Vec<Ability>,
    pub health: i32,
    pub attack: i32,
    pub max_health: i32,
    pub max_attack: i32,
    pub level: i32,
    pub exp: i32,
    pub durability: i32,
    pub class_name: String,
    pub cost: i32,
    pub id: i32,
    pub fatigued: bool,
}

impl Default for Card {
    fn default() -> Card {
        return Card {
            name: "Default".to_string(),
            id: 0,
            max_health: 0,
            max_attack: 0,
            health: 0,
            attack: 0,
            level: 0,
            exp: 0,
            durability: 0,
            class_name: "Default".to_owned(),
            abilities: Vec::new(),
            tmp_abilities: Vec::new(),
            cost: 0,
            fatigued: true,
        };
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.name,
            self.health,
            self.attack,
            self.cost
        )
    }
}
impl Card {
    pub fn init(&mut self) {
        println!("initializeing {}", &self.name);
        self.health = self.max_health.clone();
        self.attack = self.max_attack.clone();
        self.tmp_abilities = Vec::new();
    }
    //Grant exp to a card
    pub fn give_exp(&mut self, x: i32, deck: &Deck) {
        self.exp = &self.exp + x;
        //If exp is in the range to level up
        loop {
            //make sure that you dont get more levels than you can handle
            println!("self.level = {}", self.level);
            if self.exp > (self.level * 125 + 100) && self.level < 5 {
                level_up(self, &deck);
            } else {
                break;
                //TODO: figure out what do do with extra exp
                //the deck might get a slot for extra exp that is awarded on player.de_init()
            }
        }
    }
    pub fn pretty_print(&self) {
        println!(
            "[name:{}]\n[class:{}]\n[id:{}]\n[health:{}]\n[attack:{}]\n[level:{}]\n[exp:{}]\n[cost:{}]\n[fatigued:{}]\n[dura:{}]",
            &self.name,
            &self.class_name,
            &self.id,
            &self.health,
            &self.attack,
            &self.level,
            &self.exp,
            &self.cost,
            &self.fatigued,
            &self.durability
        );

        for i in &self.abilities {
            println!("\tName: {} Trigger: {}", i.name, i.trigger);

        }
    }
}

///An AbilityRaw is what a part of an ability that contains the effect.
///Each AbilityRaw contains what it does. and who it does it to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityRaw {
    pub target: String,
    pub effect: String,
}
impl Default for AbilityRaw {
    fn default() -> AbilityRaw {
        return AbilityRaw {
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

///Each ability has a name, minimum level needed to gain this ability, a list of
///AbilityRaws (executed sequentially) and the condition for when an ability
///is triggered
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
    pub level_requirement: i32,
    pub all_pick: String,
    pub ability_raws: Vec<AbilityRaw>,
    pub trigger: String,
}
impl Default for Ability {
    fn default() -> Ability {
        return Ability {
            name: "ability_1".to_string(),
            level_requirement: 0,
            all_pick: "all".to_owned(),
            ability_raws: Vec::new(),
            trigger: "on_play".to_owned(),
        };

    }
}


///A CardClass is The blueprint for how cards are generated.
///They are defined in the abilities.yaml file
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
        return CardClass {
            name: "Default".to_string(),
            ability_list: Vec::new(),
            init_health: 0,
            init_attack: 0,
            init_stats: vec![vec![0]],
            init_points: vec![vec![0]],
            level_stats: vec![vec![1, 1], vec![2, 1], vec![3, 1], vec![4, 1], vec![5, 1]],
            level_points: vec![
                vec![1, 33, 33, 33],
                vec![2, 33, 33, 33],
                vec![3, 33, 33, 33],
                vec![4, 33, 33, 33],
                vec![5, 33, 33, 33],
            ],
        };
    }
}

impl fmt::Display for CardClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/*  The player section */
///Players Are the structure that represents the human (or bot) player.
#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub deck: Deck,
    pub field: Vec<Card>,
    pub hand: Vec<Card>,
    pub graveyard: Vec<Card>,
    pub health: i32,
    pub id: i32,
    pub mana: i32,
    pub send: Sender<String>,
    pub recv: Receiver<String>,
}

impl Default for Player {
    fn default() -> Player {
        let (send, recv) = channel();
        return Player {
            name: "Default".to_owned(),
            deck: Deck::default(),
            field: Vec::new(),
            hand: Vec::new(),
            graveyard: Vec::new(),
            health: 0,
            id: 0,
            mana: 0,
            send: send,
            recv: recv,
        };
    }
}

impl Player {
    // Allows player to save deck
    pub fn de_init(&mut self) {
        //move all cards back to the deck and save deck
        self.deck.cards.extend_from_slice(&self.field);
        self.deck.cards.extend_from_slice(&self.graveyard);
        self.deck.save_to_file();
    }

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
        println!("has {} mana left", &self.mana)
    }
}

///Creates a player structure
pub fn create_player(name: String, deck: Deck) -> Player {
    let hand = Vec::new();
    let field = Vec::new();
    let graveyard = Vec::new();
    let health: i32 = 30;
    let (s, r) = channel();
    let player = Player {
        name: name,
        id: 0,
        health: health,
        deck: deck,
        hand: hand,
        field: field,
        graveyard: graveyard,
        mana: 0,
        send: s,
        recv: r,
    };
    return player;
}


/* The deck section */
///Decks contain a player's cards and the CardClasses allowed in it.
///This way you could take your deck generated with one abilities.yaml and
///play it against a different abilities.yaml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub name_of_deck: String,
    pub cards: Vec<Card>,
    pub card_classes: Vec<CardClass>,
    pub biggest_id: i32,
}

impl Default for Deck {
    fn default() -> Deck {
        return Deck {
            name_of_deck: "default".to_owned(),
            cards: Vec::new(),
            card_classes: Vec::new(),
            biggest_id: 0,
        };
    }
}

impl Deck {
    pub fn init(&mut self) {
        for i in &mut self.cards {
            i.init();
        }
    }
    pub fn print(&self) {
        println!("Name: {}", &self.name_of_deck);
        for i in &self.cards {
            i.pretty_print();
            println!("");

        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn save_to_file(&self) {
        let name = format!("{}.deck", &self.name_of_deck);
        let mut file = File::create(name).unwrap();
        let j = serde_json::to_string(&self).unwrap();
        file.write_all(j.as_bytes()).unwrap();
    }
}

//TODO: use generic to fuse these two functions
///Read in a deck
pub fn read_deck_from_file(input: String) -> Result<Deck, Box<Error>> {
    let p = format!("{}.deck", input);
    let file = File::open(p)?;
    let u: Deck = serde_json::from_reader(file)?;
    Ok(u)
}

fn read_card_class<P: AsRef<Path>>(path: P) -> Result<Vec<CardClass>, Box<Error>> {
    let file = File::open(path)?;
    let u = serde_yaml::from_reader(file)?;
    Ok(u)
}




///Read in a deck and create a card based on the deck's CardClasses
pub fn create_card<'a>(deck: &'a mut Deck) -> Card {

    //Pick a random class
    let class_i = rand::thread_rng().gen_range(0, deck.card_classes.len());

    let class: &CardClass = &deck.card_classes[class_i];
    let mut card = Card::default();

    deck.biggest_id += 1;
    card.id = deck.biggest_id.clone();

    card.name = format!("Level 0 {}", class.name);
    card.max_health = class.init_health;
    card.max_attack = class.init_attack;
    card.level = 0;
    card.durability = 10;
    card.class_name = class.name.clone();

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
    for i in 0..class.init_points.len() {
        if value < class.init_stats[i][1] + last - 1 || value == class.init_stats[i][1] + last - 1 {
            card.cost = (i + 1) as i32;
            break;
        } else {
            last += class.init_stats[i][1];
        }
    }

    if card.cost == 0 {
        println!("your init_stats values are off. total is > 100, setting card to 1 mana cost");
        card.cost = 1;
    }
    //Use up points
    //Get the total amount of points we will award


    //For each level
    for i in 0..card.cost {
        //for each stat point gained that level
        for _ in 0..class.init_stats[i as usize][2] {
            let value = rand::thread_rng().gen_range(0, 100);
            //Here we will figure out which we want

            let points = class.init_points[i as usize].clone();

            //add ability
            if value < points[1] {
                add_ability(&mut card, &class);
            }
            //add attack
            else if value < points[1] + points[2] {
                card.max_attack += 1;
            } else if value < points[1] + points[2] + points[3] {
                card.max_health += 1;
            }
        }
    }

    return card;
}

///Level up a card
pub fn level_up<'a>(mut card: &'a mut Card, deck: &'a Deck) {

    println!(
        "Leveling up {} with id {} its level {}",
        card.name,
        card.id,
        card.level
    );
    //For each level
    //for each stat point gained that level
    //find out which class this card belongs to
    //if the class doesnt match up, just lump it in the first class
    //TODO: consider making this cleaner
    let mut class_index = 0;
    for i in 0..deck.card_classes.len() {
        if deck.card_classes[i].name == card.class_name {
            class_index = i;
        }
    }
    card.level += 1;
    let class = &deck.card_classes[class_index];

    //How many levels to assign
    for _ in 1..class.level_stats[card.level as usize - 1][1] {
        let value = rand::thread_rng().gen_range(0, 100);

        //Here we will figure out which we want

        //this should be whatever leve you are.
        //Note: we already += 1'ed the level
        let points = class.level_points[card.level as usize].clone();

        //add ability
        if value < points[1] + 1 {
            add_ability(&mut card, &class);
            println!("adding an ability");
        }
        //add attack
        else if value < points[1] + points[2] + 1 {
            card.max_attack += 1;
            println!("adding an attack point");
        } else if value < points[1] + points[2] + points[3] + 1 {
            card.max_health += 1;
            println!("adding a health point");
        }
    }
}


///Give a card an ability.
fn add_ability<'a>(card: &'a mut Card, class: &'a CardClass) {
    //we check a maximum of all the cards
    println!("getting an ability");

    for _ in 0..class.ability_list.len() {
        //pick a random ability
        let value = rand::thread_rng()
            .choose(&class.ability_list)
            .unwrap()
            .clone();
        //if level_requirement doesnt disqualify the ability
        if value.level_requirement < card.level + 1 {
            card.abilities.push(value);
            break;
        }
    }
}


//TODO: investigate this with 0 cards and 0 exp
///Create an empty deck
pub fn create_deck(num_cards: i32, mut exp_to_grant: i32, deck_name: String) -> Deck {
    //Generate up some cards
    let mut card_vec: Vec<Card> = Vec::new();


    //Check if file exists
    println!("Does the abilities file exists?");
    println!("{}", Path::new("abilities.yaml").exists());

    let input = read_card_class("abilities.yaml".to_owned());

    //Check that it seems good
    if input.is_ok() {
        println!("input is good");
    } else {
        println!("Could not interpret abilities file");

    }

    let classes: Vec<CardClass> = input.unwrap();
    let mut deck = Deck {
        name_of_deck: "thing".to_owned(),
        card_classes: classes.clone(),
        ..Deck::default()
    };

    deck.name_of_deck = deck_name;

    for _ in 0..num_cards {
        card_vec.push(create_card(&mut deck));
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
            match rand::thread_rng().gen_range(1, 5) {
                1 => to_give = 100,
                2 => to_give = 225,
                3 => to_give = 350,
                4 => to_give = 475,
                5 => to_give = 600,
                _ => {}
            };
            //TODO: for some reason it wont let me borrow deck as imutable, ill fix later
            //for now i just clone deck to d and send that
            let d = deck.clone();
            deck.cards[which_card].give_exp(to_give, &d);
        } else {
            to_give = exp_to_grant;
        }
        exp_to_grant = exp_to_grant - to_give;
    }
    deck.save_to_file();

    return deck;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_give_exp() {
        let mut deck = Deck::default();
        let mut card = Card::default();
        let mut class = CardClass::default();
        class.name = "test".to_owned();
        class.ability_list.push(Ability::default());
        card.class_name = "test".to_owned();
        deck.card_classes.push(class);

        let x: i32 = 1000;
        card.give_exp(x, &deck);

        assert!(card.level == 5);

    }
    #[test]
    fn test_level_up() {
        let mut deck = Deck::default();
        let mut card = Card::default();
        let mut class = CardClass::default();
        class.name = "test".to_owned();
        class.ability_list.push(Ability::default());
        card.class_name = "test".to_owned();
        deck.card_classes.push(class);

        level_up(&mut card, &deck);
        level_up(&mut card, &deck);
        level_up(&mut card, &deck);
        level_up(&mut card, &deck);
        level_up(&mut card, &deck);

        card.pretty_print();

        assert!(card.level == 5);
    }
}

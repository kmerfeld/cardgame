use card::Card;
use deck::Deck;
use std::fmt;

// The field where you play on
pub struct Board {
    // The players
    player_1: Player,
    player_1_board: Vec<Card>,
    player_1_hand: Vec<Card>,
    player_2: Player,
    player_2_hand: Vec<Card>,
    player_2_board: Vec<Card>,
}

impl Board {
    pub fn new(p1_name: String, p2_name: String, p1_deck: Deck, p2_deck: Deck) -> Board {
        let p1 = Player{name: p1_name, deck: p1_deck};
        let p2 = Player{name: p2_name, deck: p2_deck};
        let board = Board{ 
            player_1: p1, 
            player_2: p2,
            player_1_board: Vec::new(),
            player_2_board: Vec::new(),
            player_1_hand: Vec::new(),
            player_2_hand: Vec::new()
        };


        return board;

    }


    //Print out the board state to STDOUT
    pub fn print(&self) {

        println!("Player 1 is: {}", &self.player_1.name);
        println!("In his hand is the following:");
        for i in &self.player_1_hand {
            i.pretty_print();
        }
        println!("on his board is the following:");
        for i in &self.player_1_board {
            i.pretty_print();
        }
        println!("Player 2 is: {}", &self.player_2.name);
        println!("In his hand is the following:");
        for i in &self.player_2_hand {
            i.pretty_print();
        }
        println!("on his board is the following:");
        for i in &self.player_2_board {
            i.pretty_print();
        }
    }
}

// The players who interact with the board
pub struct Player {
    name: String,
    deck: Deck,
}

// The actions that the players do
// These go "player" does "action" to "target".
// The board then decides what happens
pub struct Event {
    caster: Player,
    action: String, //This will be an action
    target: Card,
}

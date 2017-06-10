//First draft of client. no gui, no networking
extern crate rand;
use rand::Rng;

use board::*;


pub fn gameloop (mut board: Board) {
    //Current players turn
    //let mut turn: Player = board.player_1.clone();
    let mut player_1: Player = board.player_1.clone();
    let mut player_2: Player = board.player_2.clone();


    //if rand::thread().gen_range(1,2) == 2 { turn = &board.player_2;}

    let mut game_is_going: bool = true;

    /* Board initialization*/
    //shuffle both decks
    //player_1.deck = task_rng().shuffle(player_1.deck);
    //player_1.deck = task_rng().shuffle(player_1.deck);
    // draw cards for each player
    for i in 0..6 {
        println!("drawing_cards");
        draw_card(&mut player_1);
        draw_card(&mut player_2);
    }
    player_1.print();
    player_2.print();
    while game_is_going {
        game_is_going = false;
    
    
    
    
    
    

    }


    


}

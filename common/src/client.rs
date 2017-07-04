//First draft of client. no gui, no networking
// This is what the server will do in the back
extern crate rand;
use rand::Rng;
use std::io::{self, BufRead};

use board::*;




pub fn gameloop (mut board: Board) {
    //Current players turn
    //let mut turn: Player = board.player_1.clone();
    let mut player_1: Player = board.player_1.clone();
    let mut player_2: Player = board.player_2.clone();


    //if rand::thread().gen_range(1,2) == 2 { turn = &board.player_2;}

    let mut game_is_going: bool = true;
    let mut turn:bool = true;

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
        //Define who is who
        let mut current_player;
        let mut other_player;
        if turn == true { 
            current_player = &mut player_1;
            other_player = &mut player_2;
        }
        else {
            current_player = &mut player_2;
            other_player = &mut player_1;
        } 
        current_player.print();    
        //First the player draws a card
        draw_card(current_player);


        let mut doing_things: bool = true;
        while doing_things {
            //Asks what the player wants to do
            //The options are "play", "attack", "look", and "help"
            let mut line = String::new();
            let stdin = io::stdin();
            stdin.lock().read_line(&mut line).expect("Could not read line");
            println!("{}", line);

            let mut split = line.split_whitespace();
            let split: Vec<&str> = split.collect();

            if split[0] == "play" {
                //For now this will just be a 0-1 position in hand
                let x: i32 = split[1].parse().unwrap();
                current_player.play(1);
            }
            else if split[0] == "attack" {

            }
            else if split[0] == "look" {

            }

            else if split[0] == "end" {
                doing_things = false;
            }
            else {
                println!("enter a command, valid commands are \"play\", \"attack\", \"look\", and \"help\"");


            }
        }
    }





}

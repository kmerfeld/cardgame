//First draft of client. no gui, no networking
// This is what the server will do in the back
extern crate rand;
use rand::Rng;
use std::io::{self, BufRead};

use board::*;


pub fn gameloop (mut player_1: Player, mut player_2: Player) {
    //Current players turn
    //let mut turn: Player = board.player_1.clone();



    //if rand::thread().gen_range(1,2) == 2 { turn = &board.player_2;}

    let mut game_is_going: bool = true;
    let mut turn:bool = true;

    /* Board initialization*/
    //shuffle both decks
    //TODO: shuffle
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
        println!("\n#### This player's turn {}", current_player.name);
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

            let  split = line.split_whitespace();
            let split: Vec<&str> = split.collect();

            if split.is_empty()  { 
                println!("enter a command, valid commands are \"play\", \"attack\", \"look\", and \"help\"");
            }

            else if split[0] == "play" {
                //make sure that they had a second argument
                if split.len() < 2 {
                    println!("not enough arguments, try \"play 1\"");
                }
                else {
                    let index_of_card: i32 = split[1].parse().unwrap();
                    if current_player.hand.len() < index_of_card as usize {
                        println!("invalid card choice, max is {}", current_player.hand.len());
                    }
                    else {

                        current_player.play(index_of_card);

                    }
                }
            }
            else if split[0] == "attack" {

                //This should all be moved to the board section
                if split.len() < 3 {
                    println!("not enough arguments, try \"attack your_monster enemy\" to attack opponent, use 999");
                }
                else {

                    let index_attacker: i32 = split[1].parse().unwrap();
                    let index_target: i32 = split[2].parse().unwrap();

                    if current_player.field.len() < index_attacker as usize {
                        println!("invalid attacker choice, max is {}", current_player.field.len());
                    }
                    else if other_player.field.len() < index_attacker as usize {
                        println!("invalid target choice, max is {}", other_player.field.len());
                    }
                    let attacker: i32 = split[1].parse().unwrap();
                    let target: i32 = split[2].parse().unwrap();

                    //remove health
                    current_player.field[index_attacker as usize].health -= other_player.field[index_target as usize].attack;
                    other_player.field[index_target as usize].health -= current_player.field[index_attacker as usize].attack;


                    //move dead monsters to the graveyard
                    if current_player.field[index_attacker as usize].health > 1 {

                        println!("moving card to graveyard");
                        current_player.graveyard.push(current_player.field.remove(index_attacker as usize));

                    }
                    if other_player.field[index_target as usize].health >1 {
                        println!("moving card to graveyard");
                        other_player.graveyard.push(current_player.field.remove(index_target as usize));
                    }

                }
            }
            else if split[0] == "look" {
                current_player.print();
            }

            else if split[0] == "end" {
                doing_things = false;
            }
            else {
                println!("enter a command, valid commands are \"play\", \"attack\", \"look\", and \"help\"");
            }

        }
        if turn == false { turn = true; }
        else { turn = false; }
    }





}

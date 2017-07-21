//First draft of client. no gui, no networking
// This is what the server will do in the back
extern crate rand;
use rand::{Rng, thread_rng};
use std::io::{self, BufRead};
use cardgame_board::*;

use action::*;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

use std::thread;

pub fn gameloop(mut player_1: Player, mut player_2: Player, send_1: Sender<String>, recv_1: Receiver<String>, send_2: Sender<String>, recv_2: Receiver<String>) {

    //We clone these cards so we will keep the originals
    //at the end for persistance

    //Initialize tmp values
    player_1.deck.init();
    player_2.deck.init();

    println!("Starting game");
    player_1.deck.print();

    player_1.deck.save_to_file();

    //Determine the id's
    player_1.id = 1;
    player_2.id = 2;

    let mut game_is_going: bool = true;
    let mut turn: bool = true;

    /* Board initialization*/
    //shuffle both decks
    //TODO: shuffle
    let mut rng = thread_rng();
    rng.shuffle(&mut player_1.deck.cards);
    rng.shuffle(&mut player_2.deck.cards);
    // draw cards for each player
    for _ in 0..6 {
        println!("drawing_cards");
        draw_card(&mut player_1);
        draw_card(&mut player_2);
    }

    let mut curr_sender = &send_1;
    let mut curr_recv = &recv_1;

    player_1.print();
    player_2.print();

    /* Start game */
    while game_is_going {
        //Define who is who
        let mut current_player;
        let mut other_player;
        if turn == true {
            current_player = &mut player_1;
            other_player = &mut player_2;
            curr_sender = &send_1;
            curr_recv = &recv_1;

        } else {
            current_player = &mut player_2;
            other_player = &mut player_1;
            curr_sender = &send_2;
            curr_recv = &recv_2;

        }
        println!("\n#### This player's turn {}", current_player.name);

        //Get a mana point
        current_player.mana += 1;

        current_player.print();
        //First the player draws a card


        draw_card(current_player);

        //Turn starts,
        //unfatigue all cards
        for i in &mut current_player.field {
            i.fatigued = false;
        }
        for i in current_player.field.clone() {
            trigger_ability("on_turn_start".to_owned(),
                            &i.id,
                            &mut current_player,
                            &mut other_player,
                            &curr_sender,
                            &curr_recv);
        }

        let mut doing_things: bool = true;
        while doing_things {

            //Asks what the player wants to do
            //The options are "play", "attack", "look", and "help"
            let mut line = String::new();
            let stdin = io::stdin();
            stdin
                .lock()
                .read_line(&mut line)
                .expect("Could not read line");

            let split = line.split_whitespace();
            let split: Vec<&str> = split.collect();

            if split.is_empty() {
                println!("enter a command, valid commands are \"play\", \"attack\", \"look\", and \"help\"");
            } else if split[0] == "play" {
                let id: i32 = split[1].parse().unwrap();
                play_card(&id,
                          &mut current_player.hand,
                          &mut current_player.field,
                          &current_player.deck,
                          &mut current_player.mana);
                trigger_ability("on_play".to_owned(),
                                &id,
                                &mut current_player,
                                &mut other_player,
                                &curr_sender,
                                &curr_recv);
            } else if split[0] == "attack" {
                //This should all be moved to the board section
                //Make sure enough arguments were supplied
                if split.len() < 3 {
                    println!("not enough arguments, try \"attack your_monster enemy\" to attack opponent, use 999");
                } else {
                    let attacker: i32 = split[1].parse().unwrap();
                    let target: i32 = split[2].parse().unwrap();

                    attack(&attacker, &target, &mut current_player, &mut other_player);
                }
            } else if split[0] == "attack_face" {
                let mut id: i32 = split[1].parse().unwrap();
                attack_face(&mut id, &mut other_player, &mut current_player, &curr_sender, &curr_recv);


            } else if split[0] == "win" {
                game_is_going = false;
                break;
            } else if split[0] == "look" {
                current_player.print();
                println!("\n");
            } else if split[0] == "end" {
                doing_things = false;
            } else {
                println!("enter a command, valid commands are \"play\", \"attack\", \"look\", and \"help\"");
            }

        }

        for i in current_player.field.clone() {
            trigger_ability("on_turn_start".to_owned(),
                            &i.id,
                            &mut current_player,
                            &mut other_player,
                            &curr_sender,
                            &curr_recv);
        }

        if turn == false {
            turn = true;
        } else {
            turn = false;
        }
    }

    player_1.de_init();
    player_2.de_init();
    println!("Game is over");

}

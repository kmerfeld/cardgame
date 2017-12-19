//use std::io::{self, BufRead};
use cardgame_board::*;
use std::sync::mpsc::{Sender, Receiver};
//use std::thread;
/* Modify board state */

///The event parser will take a event object and decide what to do with it.
///
///draw_card action_param:
///    [0] = target (player.name)
///play_card
///    [0] = player who will play the card
///    [1] = id of card in hand
///move_card
///    [0] = player who has the card in their hand
///    [1] = id of card in hand
///attack
///attack_face
///trigger_ability
///modify_stat
///add_global_effect
///remove_global_effects
pub fn parse_event<'a>(event: Event, mut board: &'a mut Board) -> Result<String, String>{
    let out: Result<String, String> = match event.action.as_ref() {
        //"attack"  => attack(),
        "draw_card" => draw_card(&mut board, event),
        "play_card" => play_card(&mut board, event),
        "move_card"  => move_card(&mut board, event),
        //"attack_face"  => attack_face(),
        //"trigger_ability"  => trigger_ability(),
        //"add_global_effect"  => add_global_effect(),
        //"remove_global_effects"  => remove_global_effects(),
        _ =>  return Err("Invalid action".to_owned()),
    };
    return out;
}

///Allow something to draw a card
fn draw_card<'a>(board: &'a mut Board, event: Event) -> Result<String, String> {
    //Determine player
    board.add_event(event.clone());
    
    if board.player_1.name == event.action_args[0]{
        //Draw the card
        if board.player_1.deck.cards.len() > 0 {
            let topcard: Card = board.player_1.deck.cards.pop().unwrap();
            println!("{}", topcard);
            board.player_1.hand.push(topcard);
        }
        else {
            return Err("Not enough cards in deck for this".to_owned())
        }
    }
    else if board.player_2.name == event.action_args[0] {
        //Draw the card
        if board.player_2.deck.cards.len() > 0 {
            let topcard: Card = board.player_2.deck.cards.pop().unwrap();
            println!("{}", topcard);
            board.player_2.hand.push(topcard);
        }
        else {
            return Err("Not enough cards in deck for this".to_owned())
        }
    }
    Ok("Player Drew a card".to_owned())
}

///Move a card from your hand to the board
fn play_card<'a>(mut board: &'a mut Board, event: Event) -> Result<String, String> {

    &board.add_event(event.clone());
    let mut new_event = Event::default();
    {
        let id: i32 = event.action_args[1].parse().unwrap();
        let mut player = &mut Player::default();
        let mut p_number = 1;
        if &board.player_1.name == &event.action_args[0] { player = &mut board.player_1; }
        if &board.player_2.name == &event.action_args[0] { player = &mut board.player_2; p_number = 2;}
        let index = get_index(&id, &player.hand);

        if index.is_some() {
            if player.mana >= player.hand[index.unwrap() as usize].cost {
                player.mana = player.mana - player.hand[index.unwrap() as usize].cost;
                //Grant exp
                println!("giving card with id {} 100 exp",
                         player.hand[index.unwrap() as usize]);
                player.hand[index.unwrap() as usize].give_exp(100, &player.deck);
                //Create another event to send to move_card
                new_event = Event {from_player: p_number,
                                   visibility: 0,
                                   action: "move_card".to_owned(),
                                   action_args: vec![player.name.clone(), id.to_string()],
                };
                
            } else {
                return Err("Not enough mana".to_owned());
            }
        } else {
            return Err("Card Doesn't exist".to_owned());
        }
    }
    //
    let mc = move_card(&mut board, new_event);
    if mc.is_ok() {
        Ok("moving card".to_owned())
    }
    else {
        Err(format!("Didn't play card because: {}", mc.unwrap_err()))
    }
}

///Move a card from one location to another
fn move_card<'a>(board: &'a mut Board, event: Event) -> Result<String, String> {
    &board.add_event(event.clone());
    let mut player = &mut Player::default();
    if &board.player_1.name == &event.action_args[0] { player = &mut board.player_1; }
    else if &board.player_2.name == &event.action_args[0] { player = &mut board.player_2; }
    else {return Err("Player not found".to_owned());}

    let index = get_index(&event.action_args[1].parse::<i32>().unwrap(), &player.hand);
    if index.is_some() {
        player.field.push(player.hand.remove(index.unwrap() as usize));
        return Ok("Card moved".to_owned());
    }
    else {
        return Err("Card doesn't exist".to_owned());
    }

    //Check for effects of card
    //for i in current_player.field.last().unwrap().clone().abilities {
    //    //TODO:
    //}
    
}

/// Attack a player right in the face with one of your cards
fn attack_face<'a>(attacker: &'a i32, mut target: &'a mut Player, mut you: &'a mut Player) {
    let mut index: i32 = -1;
    for i in 0..you.field.len() {
        if you.field[i].id == attacker.clone() {
            index = i as i32;
        }

    }
    if index < 0 {
        println!("Monster doesnt exist");
    } else {
        trigger_ability("on_player_attacked".to_owned(),
        &index,
        &mut you,
        &mut target);
        target.health -= you.field[index as usize].attack;
    }
}

//Force two creatures to fight
fn attack<'a>(attacker: &'a i32,
                  target: &'a i32,
                  you: &'a mut Player,
                  opponent: &'a mut Player) {
    //For the fighting we shouldnt need the players, but if someone has an ability that when it
    //dies do something to the rest of the field then we need it

    //turn the ids into an index. we can assume that the cards are all on the field
    //and not in the graveyard or hand
    let mut a_index: i32 = -1;
    for i in 0..you.field.len() {
        if you.field[i].id == attacker.clone() {
            a_index = i as i32;
        }
    }
    let mut t_index: i32 = -1;
    for i in 0..opponent.field.len() {
        if opponent.field[i].id == target.clone() {
            t_index = i as i32;
        }
    }

    //Check that this is valid
    if a_index > -1 && t_index > -1 {
        let a = a_index as usize;
        let t = t_index as usize;

        if !you.field[a].fatigued {
            you.field[a].fatigued = true;

            //Check for on-combat abilities
            //trigger_abilities("on-combat".to_owned(), &mut you.field[a].clone(), &mut you, &mut opponent);
            //trigger_abilities("on-combat".to_owned(), &mut opponent.field[t].clone(), &mut opponent, &mut you);

            //actually do the dmg
            you.field[a].health -= opponent.field[t].attack;
            opponent.field[t].health -= you.field[a].attack;

            //Trigger on death

            //move bodies to the graveyard
            if you.field[a].health < 1 {
                //TODO: fix move_card
                //move_card(&attacker, &mut you.field, &mut you.graveyard);
            }
            if opponent.field[t].health < 1 {
                //TODO: fix move_card
                //move_card(&target, &mut opponent.field, &mut opponent.graveyard);
            }
        } else {
            println!("Cant attack, fatigued");
        }
    }
}

///Find the position of a card in a location from it's ID
//TODO: Find a solution to do this cleaner, Unique IDs?
fn get_index<'a>(id: &'a i32, location: &'a Vec<Card>) -> Option<i32> {
    let mut index = None;
    for i in 0..location.len() {
        if location[i].id == id.clone() {
            index = Some(i as i32)
        }
        else { println!("Not this card");
        println!("id-{}, locationid_{}", id.clone(), location[i].id);}
    }
    return index;
}


///Get info from the player
fn ask<'a>(message: String, send: &'a Sender<String>, recv: &'a Receiver<String>) -> String {

    //Tell the input thread we are ready
    let result = send.send(message);
    if !result.is_ok() { println!("Failed to write line, broken pipe"); }

    //Ask the home thread for input
    let x = recv.recv().unwrap();
    return x;
}

///Give output to the player
fn say<'a>(message: String, player: &'a Player) {
    let out = player.send.send(message);
    if !out.is_ok() { println!("Failed to send line, broken pipe"); }
}


/* Abilities */
fn trigger_ability<'a>(trigger: String,
                           id: &i32,
                           mut caster: &'a mut Player,
                           mut target_owner: &'a mut Player)
{

    //Get a reference to the card
    let mut card: Card = Card::default();
    let index_o = get_index(&id, &caster.field);
    let index_c = get_index(&id, &target_owner.field);
    if index_o.is_some() {
        card = caster.field[index_o.unwrap() as usize].clone();
    }
    if index_c.is_some() {
        card = target_owner.field[index_c.unwrap() as usize].clone();
    }


    //Check abilities
    for thing in card.clone().abilities {

        if thing.trigger == trigger {
            say(format!("activating {}", thing.name), &caster); 


            for ability in thing.ability_raws {

                //We know that the card is valid, and we have its index.
                //Things can only ever use thier effects when they are on the feild,
                //so we dont need to try and figure out where that is

                let effect: Vec<&str> = ability.effect.split_whitespace().collect();
                //attack's only valid target is another monster

                if effect[0].to_string() == "destroy" {

                    let mut found_target: bool = false;


                    while !found_target {
                        if ability.target == "self" {
                            //TODO fix move_card
                            //move_card(&id, &mut caster.field, &mut caster.graveyard);
                            found_target = true;
                        } else {
                            //Figure out what card will be destroyed
                            let input = ask("what monster do you want destroyed and what field (you/them) expecting \"20 them \"".to_owned(), &caster.send, &caster.recv);
                            println!("cancel to not use this ability");
                            let which: Vec<&str> = input.split_whitespace().collect();

                            if which[1] == "cancel" {
                                break;
                            }
                            if which[1] == "you" || which[1] == "them" {
                                //target creature on your side
                                if which[1] == "you" &&
                                    (ability.target == "target_creature" ||
                                     ability.target == "target_ally_creature") {

                                        let index_c =
                                            get_index(&which[0].trim().parse::<i32>().unwrap(),
                                            &caster.field);

                                        //TODO: fix move_card
                                        /*
                                        if index_c.is_some() {
                                            if ability.target == "target creature".to_owned() ||
                                                ability.target == "ally creature".to_owned() {
                                                    move_card(&which[0].trim().parse::<i32>().unwrap(),
                                                    &mut caster.field,
                                                    &mut caster.graveyard);
                                                    trigger_ability("on_death".to_owned(),
                                                    &which[0]
                                                    .trim()
                                                    .parse::<i32>()
                                                    .unwrap(),
                                                    &mut caster,
                                                    &mut target_owner);
                                                    found_target = true;
                                                }
                                        }
                                        */
                                    }
                                //target creature on thier side
                                else if which[1] == "them" &&
                                    (ability.target == "target_creature" ||
                                     ability.target == "target_enemy_creature") {
                                        let index_t =
                                            get_index(&which[0].trim().parse::<i32>().unwrap(),
                                            &target_owner.field);

                                        //TODO: Fix move_card
                                        /*
                                        //If its on the opponents side
                                        if index_t.is_some() {
                                            if ability.target == "target enemy creature".to_owned() {
                                                move_card(&which[0].trim().parse::<i32>().unwrap(),
                                                &mut target_owner.field,
                                                &mut target_owner.graveyard);
                                                trigger_ability("on_death".to_owned(),
                                                &which[0]
                                                .trim()
                                                .parse::<i32>()
                                                .unwrap(),
                                                &mut target_owner,
                                                &mut caster);
                                                found_target = true;
                                            }
                                        }
                                        */
                                    }
                                //Check both fields
                                else {
                                    println!("invalid target. ");
                                }
                            }
                        }
                    }
                }
                //Modify ability
                else if effect[0].to_string().contains("modify") {
                    //Check if changes will be permanant
                    let perm: bool = effect[0].contains("permanant");
                    println!("is it perm? {}", perm);

                    //effect[1] is the stat we are looking to change,
                    //effect[2] is by how much we will change it
                    if ability.target == "self" {
                        if index_o.is_some() {
                            modify_stat(perm,
                                        &id,
                                        effect[1].to_owned(),
                                        effect[2].parse().unwrap_or(0),
                                        &mut caster.field,
                                        &mut caster.graveyard);

                        }
                    } else if ability.target.contains("target_creature") {
                        let tmp_id: String = ask("What id do you want to target".to_owned(), &caster.send, &caster.recv);
                        let target_id = tmp_id.parse::<i32>();

                        if target_id.is_ok() {
                            let is_ally = get_index(&target_id.clone().unwrap(), &caster.field);
                            if is_ally.is_some() {
                                modify_stat(perm,
                                            &id,
                                            effect[1].to_owned(),
                                            effect[2].parse().unwrap_or(0),
                                            &mut caster.field,
                                            &mut caster.graveyard);
                            } else {
                                let is_ally = get_index(&target_id.unwrap(), &caster.field);
                                if is_ally.is_some() {
                                    modify_stat(perm,
                                                &id,
                                                effect[1].to_owned(),
                                                effect[2].parse().unwrap_or(0),
                                                &mut target_owner.field,
                                                &mut caster.graveyard);
                                }
                            }
                        }
                    } else if ability.target == "target_ally_creature" {
                        let tmp_id: String = ask("What id do you want to target".to_owned(), &caster.send, &caster.recv);
                        let target_id = tmp_id.parse::<i32>();

                        if target_id.is_ok() {
                            let is_ally = get_index(&target_id.clone().unwrap(), &caster.field);
                            if is_ally.is_some() {
                                modify_stat(perm,
                                            &id,
                                            effect[1].to_owned(),
                                            effect[2].parse().unwrap_or(0),
                                            &mut caster.field,
                                            &mut caster.graveyard);
                            }
                        }
                    } else if ability.target == "target_enemy_creature" {
                        let tmp_id: String = ask("What id do you want to target".to_owned(), &caster.send, &caster.recv);
                        let target_id = tmp_id.parse::<i32>();

                        if target_id.is_ok() {
                            let is_ally = get_index(&target_id.unwrap(), &caster.field);
                            if is_ally.is_some() {
                                modify_stat(perm,
                                            &id,
                                            effect[1].to_owned(),
                                            effect[2].parse().unwrap_or(0),
                                            &mut target_owner.field,
                                            &mut target_owner.graveyard);
                            }
                        }
                    } else if ability.target == "both_fields" {
                        for i in 0..caster.field.len() {
                            modify_stat(perm,
                                        &&caster.field[i].id.clone(),
                                        effect[1].to_owned(),
                                        effect[2].parse().unwrap_or(0),
                                        &mut caster.field,
                                        &mut caster.graveyard);
                        }
                        for i in 0..target_owner.field.len() {
                            modify_stat(perm,
                                        &&target_owner.field[i].id.clone(),
                                        effect[1].to_owned(),
                                        effect[2].parse().unwrap_or(0),
                                        &mut target_owner.field,
                                        &mut target_owner.graveyard);
                        }
                    } else if ability.target == "enemy_field" {
                        for i in 0..target_owner.field.len() {
                            modify_stat(perm,
                                        &&target_owner.field[i].id.clone(),
                                        effect[1].to_owned(),
                                        effect[2].parse().unwrap_or(0),
                                        &mut target_owner.field,
                                        &mut target_owner.graveyard);
                        }

                    } else if ability.target == "ally_field" {
                        for i in 0..caster.field.len() {
                            modify_stat(perm,
                                        &&caster.field[i].id.clone(),
                                        effect[1].to_owned(),
                                        effect[2].parse::<i32>().unwrap(),
                                        &mut caster.field,
                                        &mut caster.graveyard);
                        }
                    }
                }
            }
        }
    }
}

fn modify_stat<'a>(permanant: bool,
                       id: &'a &i32,
                       stat: String,
                       amount: i32,
                       mut location: &'a mut Vec<Card>,
                       mut graveyard: &'a mut Vec<Card>) {
    let index = get_index(id, location);
    if index.is_some() {
        //Permanantly modify a stat
        if stat == "health" {
            //Modify temp health
            location[index.unwrap() as usize].health += amount;
            //Modify permanant health
            if permanant {
                location[index.unwrap() as usize].max_health += amount;
            }
            // kill the creature if needed
            if location[index.unwrap() as usize].health < 1 {
                println!("a card died");
                //TODO: renable this
                //move_card(&id, &mut location, &mut graveyard);
            }
        } else if stat == "attack" {
            location[index.unwrap() as usize].attack += amount;
            if permanant {
                location[index.unwrap() as usize].max_attack += amount;
            }
        } else if stat == "durability" && permanant {
            location[index.unwrap() as usize].durability += amount;
        }
    } else {
        println!("Could not find creature");
    }
}

#[cfg(test)]
mod tests {
    use action::*;
    //use cardgame_board::*;

    /*
    #[test]
    fn test_ability_modify() {
        let a: AbilityRaw = AbilityRaw {
            target: "enemy_field".to_owned(),
            effect: "modify attack 5".to_owned(),
        };
        let b: AbilityRaw = AbilityRaw {
            target: "enemy_field".to_owned(),
            effect: "modify health 5".to_owned(),
        };

        let mut buff: Ability = Ability::default();
        buff.ability_raws.push(b);
        buff.ability_raws.push(a);

        let mut card: Card = Card {
            name: "Test_card".to_owned(),
            id: 1,
            ..Card::default()
        };

        let card1 = Card {
            attack: 0,
            id: 2,
            ..Card::default()
        };
        let mut card2: Card = card1.clone();
        card2.id = 3;

        card.abilities.push(buff);

        let d: Deck = Deck {
            cards: Vec::new(),
            name_of_deck: "deck".to_owned(),
            ..Deck::default()
        };
        let mut p1: Player = create_player("p1".to_owned(), d.clone());
        let mut p2: Player = create_player("p2".to_owned(), d.clone());

        p1.field.push(card.clone());
        p2.field.push(card1);
        p2.field.push(card2);

        trigger_ability("on_play".to_owned(), &card.id, &mut p1, &mut p2);
        assert!(p2.field[0].attack == 5);
        assert!(p2.field[1].attack == 5);
        assert!(p2.field[0].health == 5);
        assert!(p2.field[1].health == 5);
    }
    #[test]
    fn test_permant_modify() {
        let a: AbilityRaw = AbilityRaw {
            target: "ally_field".to_owned(),
            effect: "permanantly_modify health 5".to_owned(),
        };

        let mut buff: Ability = Ability::default();
        buff.ability_raws.push(a);

        let mut card: Card = Card {
            name: "Test_card".to_owned(),
            id: 1,
            ..Card::default()
        };

        card.abilities.push(buff);

        let d: Deck = Deck {
            cards: Vec::new(),
            name_of_deck: "deck".to_owned(),
            ..Deck::default()
        };
        let mut p1: Player = create_player("p1".to_owned(), d.clone());
        let mut p2: Player = create_player("p1".to_owned(), d.clone());

        p1.field.push(card.clone());

    
        trigger_ability("on_play".to_owned(), &card.id, &mut p1, &mut p2);
        assert!(p1.field[0].health == 5);
        assert!(p1.field[0].max_health == 5);
    }
    */
    /*
    #[test]
    fn test_attack() {
        let card: Card = Card {
            name: "Test_card".to_owned(),
            fatigued: false,
            id: 1,
            attack: 1,
            health: 1,
            ..Card::default()
        };
        let d: Deck = Deck {
            cards: Vec::new(),
            name_of_deck: "deck".to_owned(),
            ..Deck::default()
        };
        let mut p1: Player = create_player("p1".to_owned(), d.clone());
        let mut p2: Player = create_player("p1".to_owned(), d.clone());
        p1.field.push(card.clone());
        p2.field.push(card.clone());

        attack(&1, &1, &mut p1, &mut p2);

        assert!(p1.graveyard.len() == 1);
    }
    */

    //pub fn attack_face<'a>(attacker: &'a i32, mut target: &'a mut Player, mut you: &'a mut Player) {
    /*
    #[test]
    fn test_attack_face() {
        let card: Card = Card {
            name: "Test_card".to_owned(),
            fatigued: false,
            id: 1,
            attack: 1,
            health: 1,
            ..Card::default()
        };
        let d: Deck = Deck {
            cards: Vec::new(),
            name_of_deck: "deck".to_owned(),
            ..Deck::default()
        };
        let mut p1: Player = create_player("p1".to_owned(), d.clone());
        let mut p2: Player = create_player("p1".to_owned(), d.clone());
        p1.field.push(card.clone());
        p2.health = 30;

        attack_face(&1, &mut p2, &mut p1);

        assert!(p2.health == 29);
    }
    */

    #[test]
    fn test_draw_card() {
        //Setup
        let mut b = Board::default();
        let p1 = Player::default();
        let x = b.add_player(p1);
        assert!(x.is_ok());
        let c = Card::default();
        b.player_1.deck.cards.push(c);

        //Test Draw event works
        let a_p = vec!["Default".to_owned()];
        let e = Event{from_player: 1, visibility: 1, action: "draw_card".to_owned(), action_args: a_p};
        assert!(parse_event(e, &mut b).is_ok());
        assert!(b.player_1.hand.len() == 1);
    }
    #[test]
    fn test_play_card() {
        //Setup
        let mut b = Board::default();
        let mut p1 = Player::default();
        p1.name = "Test_user".to_owned();
        let x = b.add_player(p1);
        assert!(x.is_ok());
        let mut c = Card::default();
        c.id = 27;
        b.player_1.hand.push(c);

        //Test Draw event works
        let a_p = vec!["Test_user".to_owned(), "27".to_owned()];
        let e = Event{from_player: 1, visibility: 1, action: "play_card".to_owned(), action_args: a_p};
        let x = parse_event(e, &mut b);
        println!("Parse output: {}", x.unwrap());;
        //assert!(parse_event(e, &mut b).is_ok());
        assert!(b.player_1.field.len() == 1);


    }


    #[test]
    fn test_move_card() {
        let mut b = Board::default();
        let p1 = Player::default();
        let x = b.add_player(p1);
        assert!(x.is_ok());
        let mut c = Card::default();
        c.id = 27;
        b.player_1.hand.push(c);
        println!("Cards in hand {}", b.player_1.hand.len());

        let a_p = vec!["Default".to_owned(), "27".to_owned()];
        let e = Event{from_player: 1, visibility: 1, action: "move_card".to_owned(), action_args: a_p};
        assert!(parse_event(e, &mut b).is_ok());
        assert!(b.player_1.field.len() == 1 && b.player_1.hand.len() == 0);

    }
}

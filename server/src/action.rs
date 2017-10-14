use std::io::{self, BufRead};
use cardgame_board::*;
use std::thread;
use cardgame_board::*;
/* Modify board state */

///Allow a player to draw a card
pub fn draw_card<'a>(player: &'a mut Player) {
    if player.deck.cards.len() > 0 {
        let topcard: Card = player.deck.cards.pop().unwrap();
        println!("{}", topcard);
        player.hand.push(topcard);
    }
}

///Play a card from your hand to the field
pub fn play_card<'a>(
    id: &'a i32,
    mut curr_loc: &'a mut Vec<Card>,
    mut destination: &'a mut Vec<Card>,
    deck: &Deck,
    mut mana: &'a mut i32,
) {
    let index = get_index(&id, &curr_loc);
    if index.is_some() {
        if mana.clone() + 1 > curr_loc[index.unwrap() as usize].cost {
            //TODO:find out a cleaner way to do this
            let mut x = mana.clone();
            x -= curr_loc[index.unwrap() as usize].cost;
            mana.clone_from(&mut x.clone());
            //Grant exp
            println!(
                "giving card with id {} 100 exp",
                curr_loc[index.unwrap() as usize]
            );
            curr_loc[index.unwrap() as usize].give_exp(100, &deck);
            move_card(&id, &mut curr_loc, &mut destination);
        } else {
            println!("Not enough mana");
        }
    } else {
        println!("Card doesnt exist");
    }
}

///Move a card from one location to another
pub fn move_card<'a>(
    id: &'a i32,
    mut curr_loc: &'a mut Vec<Card>,
    mut destination: &'a mut Vec<Card>,
) {
    //find the index
    let mut card: i32 = -1;
    for i in 0..curr_loc.len() {
        if curr_loc[i].id == id.clone() {
            card = i as i32;
            break;
        }
    }
    if card == -1 {
        println!("Card doesnt exist");
    } else {
        //actually move the card
        destination.push(curr_loc.remove(card as usize));

        //Check for effects of card
        //for i in current_player.field.last().unwrap().clone().abilities {
        //    //TODO:
        //}
    }
}


/// Attack a player right in the face with one of your cards
pub fn attack_face<'a>(attacker: &'a i32, mut target: &'a mut Player, mut you: &'a mut Player) {
    let mut index: i32 = -1;
    for i in 0..you.field.len() {
        if you.field[i].id == attacker.clone() {
            index = i as i32;
        }

    }
    if index < 0 {
        println!("Monster doesnt exist");
    } else {
        trigger_ability(
            "on_player_attacked".to_owned(),
            &index,
            &mut you,
            &mut target,
        );
        target.health -= you.field[index as usize].attack;
    }
}

///Force two creatures to fight
pub fn attack<'a>(
    attacker: &'a i32,
    target: &'a i32,
    mut you: &'a mut Player,
    mut opponent: &'a mut Player,
) {
    //For the fighting we shouldn't need the players, but if someone has an ability that when it
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
                move_card(&attacker, &mut you.field, &mut you.graveyard);
            }
            if opponent.field[t].health < 1 {
                move_card(&target, &mut opponent.field, &mut opponent.graveyard);
            }
        } else {
            println!("Cant attack, fatigued");
        }
    }
}


///Find the position a card is in on the field
fn get_index<'a>(id: &'a i32, location: &'a Vec<Card>) -> Option<i32> {
    let mut index = None;
    for i in 0..location.len() {
        if location[i].id == id.clone() {
            index = Some(i as i32)

        }
    }
    return index;
}


///Interact with the player
///This might end up moved
fn ask<'a>(message: String) -> String {

    println!("{}", message);

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect(
        "Failed to read line",
    );
    return input;
}


/* Abilities */
///Init an ability
pub fn trigger_ability<'a>(
    trigger: String,
    id: &i32,
    mut caster: &'a mut Player,
    mut target_owner: &'a mut Player,
) {

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
            println!("activating {}", thing.name);


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
                            move_card(&id, &mut caster.field, &mut caster.graveyard);
                            found_target = true;
                        } else {
                            //Figure out what card will be destroyed
                            let input = ask("what monster do you want destroyed and what field (you/them) expecting \"20 them \"".to_owned());
                            println!("cancel to not use this ability");
                            let which: Vec<&str> = input.split_whitespace().collect();

                            if which[1] == "cancel" {
                                break;
                            }
                            if which[1] == "you" || which[1] == "them" {
                                //target creature on your side
                                if which[1] == "you" &&
                                    (ability.target == "target_creature" ||
                                         ability.target == "target_ally_creature")
                                {

                                    let index_c = get_index(
                                        &which[0].trim().parse::<i32>().unwrap(),
                                        &caster.field,
                                    );

                                    if index_c.is_some() {
                                        if ability.target == "target creature".to_owned() ||
                                            ability.target == "ally creature".to_owned()
                                        {
                                            move_card(
                                                &which[0].trim().parse::<i32>().unwrap(),
                                                &mut caster.field,
                                                &mut caster.graveyard,
                                            );
                                            trigger_ability(
                                                "on_death".to_owned(),
                                                &which[0].trim().parse::<i32>().unwrap(),
                                                &mut caster,
                                                &mut target_owner,
                                            );
                                            found_target = true;
                                        }
                                    }
                                }
                                //target creature on thier side
                                else if which[1] == "them" &&
                                           (ability.target == "target_creature" ||
                                                ability.target == "target_enemy_creature")
                                {
                                    let index_t = get_index(
                                        &which[0].trim().parse::<i32>().unwrap(),
                                        &target_owner.field,
                                    );

                                    //If its on the opponents side
                                    if index_t.is_some() {
                                        if ability.target == "target enemy creature".to_owned() {
                                            move_card(
                                                &which[0].trim().parse::<i32>().unwrap(),
                                                &mut target_owner.field,
                                                &mut target_owner.graveyard,
                                            );
                                            trigger_ability(
                                                "on_death".to_owned(),
                                                &which[0].trim().parse::<i32>().unwrap(),
                                                &mut target_owner,
                                                &mut caster,
                                            );
                                            found_target = true;
                                        }
                                    }
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
                            modify_stat(
                                perm,
                                &id,
                                effect[1].to_owned(),
                                effect[2].parse().unwrap_or(0),
                                &mut caster.field,
                                &mut caster.graveyard,
                            );

                        }
                    } else if ability.target.contains("target_creature") {
                        let tmp_id: String = ask("What id do you want to target".to_owned());
                        let target_id = tmp_id.parse::<i32>();

                        if target_id.is_ok() {
                            let is_ally = get_index(&target_id.clone().unwrap(), &caster.field);
                            if is_ally.is_some() {
                                modify_stat(
                                    perm,
                                    &id,
                                    effect[1].to_owned(),
                                    effect[2].parse().unwrap_or(0),
                                    &mut caster.field,
                                    &mut caster.graveyard,
                                );
                            } else {
                                let is_ally = get_index(&target_id.unwrap(), &caster.field);
                                if is_ally.is_some() {
                                    modify_stat(
                                        perm,
                                        &id,
                                        effect[1].to_owned(),
                                        effect[2].parse().unwrap_or(0),
                                        &mut target_owner.field,
                                        &mut caster.graveyard,
                                    );
                                }
                            }
                        }
                    } else if ability.target == "target_ally_creature" {
                        let tmp_id: String = ask("What id do you want to target".to_owned());
                        let target_id = tmp_id.parse::<i32>();

                        if target_id.is_ok() {
                            let is_ally = get_index(&target_id.clone().unwrap(), &caster.field);
                            if is_ally.is_some() {
                                modify_stat(
                                    perm,
                                    &id,
                                    effect[1].to_owned(),
                                    effect[2].parse().unwrap_or(0),
                                    &mut caster.field,
                                    &mut caster.graveyard,
                                );
                            }
                        }
                    } else if ability.target == "target_enemy_creature" {
                        let tmp_id: String = ask("What id do you want to target".to_owned());
                        let target_id = tmp_id.parse::<i32>();

                        if target_id.is_ok() {
                            let is_ally = get_index(&target_id.unwrap(), &caster.field);
                            if is_ally.is_some() {
                                modify_stat(
                                    perm,
                                    &id,
                                    effect[1].to_owned(),
                                    effect[2].parse().unwrap_or(0),
                                    &mut target_owner.field,
                                    &mut target_owner.graveyard,
                                );
                            }
                        }
                    } else if ability.target == "both_fields" {
                        for i in 0..caster.field.len() {
                            modify_stat(
                                perm,
                                &&caster.field[i].id.clone(),
                                effect[1].to_owned(),
                                effect[2].parse().unwrap_or(0),
                                &mut caster.field,
                                &mut caster.graveyard,
                            );
                        }
                        for i in 0..target_owner.field.len() {
                            modify_stat(
                                perm,
                                &&target_owner.field[i].id.clone(),
                                effect[1].to_owned(),
                                effect[2].parse().unwrap_or(0),
                                &mut target_owner.field,
                                &mut target_owner.graveyard,
                            );
                        }
                    } else if ability.target == "enemy_field" {
                        for i in 0..target_owner.field.len() {
                            modify_stat(
                                perm,
                                &&target_owner.field[i].id.clone(),
                                effect[1].to_owned(),
                                effect[2].parse().unwrap_or(0),
                                &mut target_owner.field,
                                &mut target_owner.graveyard,
                            );
                        }

                    } else if ability.target == "ally_field" {
                        for i in 0..caster.field.len() {
                            modify_stat(
                                perm,
                                &&caster.field[i].id.clone(),
                                effect[1].to_owned(),
                                effect[2].parse::<i32>().unwrap(),
                                &mut caster.field,
                                &mut caster.graveyard,
                            );
                        }
                    }
                }
            }
        }
    }
}

///Increase or decrease health, attack, or durability
pub fn modify_stat<'a>(
    permanant: bool,
    id: &'a &i32,
    stat: String,
    amount: i32,
    mut location: &'a mut Vec<Card>,
    mut graveyard: &'a mut Vec<Card>,
) {
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
                move_card(&id, &mut location, &mut graveyard);
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
    use cardgame_board::*;

    #[test]
    fn test_move_card() {
        let card: Card = Card {
            name: "Test_card".to_owned(),
            ..Card::default()
        };
        let mut hand: Vec<Card> = Vec::new();
        let mut field: Vec<Card> = Vec::new();
        hand.push(card.clone());
        move_card(&card.id, &mut hand, &mut field);
        assert!(field.last().unwrap().name == "Test_card");
    }

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

        let card1 = Card {
            max_health: 0,
            health: 0,
            id: 2,
            ..Card::default()
        };

        card.abilities.push(buff);


        let mut d: Deck = Deck {
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

    #[test]
    fn test_attack() {
        let mut card: Card = Card {
            name: "Test_card".to_owned(),
            fatigued: false,
            id: 1,
            attack: 1,
            health: 1,
            ..Card::default()
        };
        let mut d: Deck = Deck {
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

    //pub fn attack_face<'a>(attacker: &'a i32, mut target: &'a mut Player, mut you: &'a mut Player) {
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
}

use board::*;
use std::io::{self, BufRead};

/* Modify board state */

//Allow something to draw a card
pub fn draw_card<'a>(player: &'a mut Player, max_id: &'a mut i32) {
    if player.deck.cards.len() > 0 {
        let mut topcard: Card = player.deck.cards.pop().unwrap();
        *max_id += 1;
        topcard.id = max_id.clone();
        println!("{}", topcard);
        player.hand.push(topcard);
    }
}

//Move a card from one location to another
pub fn move_card<'a>(id: &'a i32, curr_loc: &'a mut Vec<Card>, destination: &'a mut Vec<Card>) { 
    //find the index
    let mut card: i32 = -1;
    for i in 0..curr_loc.len() {
        if curr_loc[i].id == id.clone() {
            card = i as i32;
            break;
        }   
    }
    if card == -1{
        println!("Card doesnt exist");
    }
    else {
        //actually move the card
        destination.push(curr_loc.remove(card as usize));

        //Check for effects of card
        //for i in current_player.field.last().unwrap().clone().abilities {
        //    //TODO:
        //}
    }
}


// Attack a player right in the face with one of your cards
pub fn attack_face<'a>(attacker: &'a mut i32, mut target: &'a mut Player, mut you: &'a mut Player) {
    let mut index: i32 = -1;
    for i in 0..you.field.len() {
        if you.field[i].id == attacker.clone() { index = i as i32; }

    }
    if index < 0 { 
        println!("Monster doesnt exist");} 
    else {
        //trigger_abilities("on_player_attacked".to_owned(), &mut you.field[index as usize].clone(), &mut you, &mut target);

        target.health -= you.field[index as usize].attack;
    }
}

//Force two creatures to fight
pub fn attack<'a>(attacker: &'a i32, target: &'a i32, mut you: &'a mut Player, mut opponent: &'a mut Player) {
    //For the fighting we shouldnt need the players, but if someone has an ability that when it
    //dies do something to the rest of the field then we need it

    //turn the ids into an index. we can assume that the cards are all on the field
    let mut a_index: i32 = -1;
    for i in 0..you.field.len() {
        if you.field[i].id == attacker.clone() { a_index = i as i32; }
    }
    let mut t_index: i32 = -1;
    for i in 0..opponent.field.len() {
        if opponent.field[i].id == target.clone() { t_index = i as i32; }
    }

    //Check that this is valid
    if a_index > -1 && t_index > -1 {
        let a = a_index as usize;
        let t = t_index as usize;

        if !you.field[a].fatigued {

            //Check for on-combat abilities
            //trigger_abilities("on-combat".to_owned(), &mut you.field[a].clone(), &mut you, &mut opponent);
            //trigger_abilities("on-combat".to_owned(), &mut opponent.field[t].clone(), &mut opponent, &mut you);

            //actually do the dmg
            you.field[a].health -= opponent.field[t].attack;
            opponent.field[t].health -= you.field[a].attack;

            //Trigger on death
            //trigger_abilities("on-combat".to_owned(), &mut you.field[a].clone(), &mut you, &mut opponent);
            //trigger_abilities("on-combat".to_owned(), &mut opponent.field[t].clone(), &mut opponent, &mut you);

            //move bodies to the graveyard
            if you.field[a].health < 1 { move_card(&attacker, &mut you.field, &mut you.graveyard); }
            if opponent.field[t].health < 1 { move_card(&target, &mut opponent.field, &mut opponent.graveyard); }
        }
    }
}


fn get_index<'a>(id: &'a i32, location: &'a Vec<Card>) -> Option<i32> {
    let mut index = None;
    for i in 0..location.len() {
        if location[i].id == id.clone() { 
            index = Some(i as i32)

        }
    }
    return index;
}


fn ask(message: String) -> String {
    //This will be sent to the current player
    println!("{}", message);
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).expect("could not read line");
    return line.trim_right_matches("\r\n").to_string();
}
/* Abilities */

pub fn trigger_single<'a>(trigger: String, id: &i32, caster: &'a mut Player, target_owner: &'a mut Player) {

    //Get a reference to the card
    let mut card: Card = Card::default();
    let index_o = get_index(&id, &caster.field);
    let index_c = get_index(&id, &target_owner.field);
    if index_o.is_some() { card = caster.field[index_o.unwrap() as usize].clone();  }
    if index_c.is_some() { card = target_owner.field[index_c.unwrap()as usize].clone(); }


    //Check abilities
    for ability in card.clone().abilities {
        //If its the correct trigger type
        if ability.trigger == trigger {
            //We know that the card is valid, and we have its index.
            //Things can only ever use thier effects when they are on the feild,
            //so we dont need to try and figure out where that is

            let effect:Vec<&str> = ability.effect.split_whitespace().collect();
            //attack's only valid target is another monster

            if effect[0].to_string() == "destroy" {
                //Figure out what card will be destroyed
                let t = ask("what monster do you want destroyed".to_owned());

                //Check both fields
                let index_c = get_index(&t.trim().parse::<i32>().unwrap(), &caster.field);
                let index_t = get_index(&t.trim().parse::<i32>().unwrap(), &target_owner.field);


                //if its your side
                if index_c.is_some() { 
                    if ability.target == "target creature".to_owned() || ability.target == "ally creature".to_owned() {
                        move_card(&t.trim().parse::<i32>().unwrap(), &mut caster.field, &mut caster.graveyard);
                        //trigger_single("on_death".to_owned(), &id, &'a );
                    }
                }

                //If its on the opponents side
                else if index_t.is_some() { 
                    if ability.target == "target enemy creature".to_owned() {
                        move_card(&t.trim().parse::<i32>().unwrap(), &mut target_owner.field, &mut target_owner.graveyard);
                        //trigger_single("on_death".to_owned());
                    }
                }
                else {
                    println!("invalid target");
                    break;
                }
            }
            //Modify ability
            else if effect[0].to_string() == "modify" {
                //effect[1] is the stat we are looking to change,
                //effect[2] is by how much we will change it
                if ability.target == "self" {
                    if index_o.is_some() {
                        modify_stat(&id, effect[1].to_owned(), effect[2].parse().unwrap_or(0), &mut caster.field);
                    }
                }
                else if ability.target == "target_creature" {
                    let tmp_id: String = ask("What id do you want to target".to_owned());
                    let target_id = tmp_id.parse::<i32>();

                    if target_id.is_ok() {
                        let is_ally = get_index(&target_id.clone().unwrap(), &caster.field);
                        if is_ally.is_some() {
                            modify_stat(&id, effect[1].to_owned(), effect[2].parse().unwrap_or(0), &mut caster.field);
                        }
                        else {
                            let is_ally = get_index(&target_id.unwrap(), &caster.field);
                            if is_ally.is_some() {
                                modify_stat(&id, effect[1].to_owned(), effect[2].parse().unwrap_or(0), &mut target_owner.field);
                            }
                        }
                    }
                }
                else if ability.target == "target_ally_creature" {
                    let tmp_id: String = ask("What id do you want to target".to_owned());
                    let target_id = tmp_id.parse::<i32>();

                    if target_id.is_ok() {
                        let is_ally = get_index(&target_id.clone().unwrap(), &caster.field);
                        if is_ally.is_some() {
                            modify_stat(&id, effect[1].to_owned(), effect[2].parse().unwrap_or(0), &mut caster.field);
                        }
                    }
                }
                else if ability.target == "target_enemy_creature" {
                    let tmp_id: String = ask("What id do you want to target".to_owned());
                    let target_id = tmp_id.parse::<i32>();

                    if target_id.is_ok() {
                        let is_ally = get_index(&target_id.unwrap(), &caster.field);
                        if is_ally.is_some() {
                            modify_stat(&id, effect[1].to_owned(), effect[2].parse().unwrap_or(0), &mut target_owner.field);
                        }
                    }
                }
                else if ability.target == "both_fields" {
                    for i  in 0..caster.field.len() {
                        modify_stat(&&caster.field[i].id.clone(), effect[1].to_owned(), effect[2].parse().unwrap_or(0), &mut caster.field);
                    }
                    for i in 0..target_owner.field.len() {
                        modify_stat(&&target_owner.field[i].id.clone(), effect[1].to_owned(), effect[2].parse().unwrap_or(0), &mut target_owner.field);
                    }
                }
                else if ability.target == "enemy_field" {
                    for i in 0..target_owner.field.len() {
                        modify_stat(&&target_owner.field[i].id.clone(), effect[1].to_owned(), effect[2].parse().unwrap_or(0), &mut target_owner.field);
                    }

                }
                else if ability.target == "ally_field" {
                    for i in 0..caster.field.len() {
                        modify_stat(&&caster.field[i].id.clone(), effect[1].to_owned(), effect[2].parse::<i32>().unwrap(), &mut caster.field);
                    }
                }
            }
        }
    }
}

pub fn modify_stat<'a>(id: &'a &i32, stat: String, amount: i32, location: &'a mut Vec<Card>) {
    let index = get_index(id, location);
    if index.is_some() {
        if stat == "health" { location[index.unwrap() as usize].health += amount; }
        else if stat == "attack" { location[index.unwrap() as usize].attack += amount; }
        else if stat == "durability" { location[index.unwrap() as usize].durability += amount; }
    }
    else {
        println!("Could not find creature");
    }
}


//Checks all creatures on a field
pub fn trigger_player<'a>(trigger: String, you: &'a mut Player, opponent: &'a mut Player) {}
//This will have the following triggers,
//on player_attacked
//on turn_start
//on turn_end



pub fn untill_turn_start<'a>(mut player: &'a mut Player) {

    player.field[0].abilities.remove(0);

    for i in &mut player.field {
        for j in 0..i.abilities.len() {
            if i.abilities[j].trigger == "untill_turn_start".to_owned() {
                i.abilities.remove(j);
            }
        }
    }
}
pub fn untill_turn_end<'a>(mut player: &'a mut Player) {

    player.field[0].abilities.remove(0);

    for i in &mut player.field {
        for j in 0..i.abilities.len() {
            if i.abilities[j].trigger == "untill_turn_end".to_owned() {
                i.abilities.remove(j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use action::*;
    use board::*;

    #[test]
    fn test_move_card() {
        let card: Card = Card{name: "Test_card".to_owned(),  ..Card::default()};
        let mut hand: Vec<Card> = Vec::new();
        let mut field: Vec<Card> = Vec::new();
        hand.push(card.clone());
        move_card(&card.id, &mut hand, &mut field);
        assert!(field.last().unwrap().name == "Test_card");
    }
}


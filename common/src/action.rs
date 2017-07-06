use board::*;

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

//Play a card from your hand to the field
pub fn play<'a>(split: Vec<&str>, current_player: &'a mut Player, other_player: &'a mut Player) { 
    //make sure that they had a second argument
    if split.len() < 2 {
        println!("not enough arguments, try \"play 1\"");
    }
    else {
        let id: i32 = split[1].parse().unwrap();
        
        let mut card: i32 = -1;
        //Check if you have the card with the index your are looking for
        for i in 0..current_player.hand.len() {
            if current_player.hand[i].id == id {
                card = i as i32;
                break;
            }
        }

        //Check if you have the card
        if card == -1 {
            println!("Card doesnt exists");
        }
        //Play the card
        else {
            current_player.field.push(current_player.hand.remove(card as usize));
        }

        //Check for effects of card
        for i in current_player.field.last.abilities {
            //TODO:
        }

    }
}

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

//Move a card from the field to graveyard
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


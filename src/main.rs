extern crate rand;
#[macro_use] extern crate text_io;
#[macro_use] extern crate serde_derive;

use rand::Rng;
mod Card;
mod Deck;

//fn lines_from_file<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>, io::Error>
//    where P: AsRef<Path>
//{
//    let mut file = try!(File::open(filename));
//    Ok(io::BufReader::new(file).lines())
//}
fn main() {
    let mut t = Vec::new();


    // ask how many cards to make
    println!("How many cards do you want?");
    let num_cards: i32 = read!();
    println!("How much exp do you want to grant? ");
    println!("It takes about 100 to go from level 0 to 1");
    let mut exp_granting: i32 = read!();



    //Generate up some cards
    for _ in 0..num_cards {

        let x = Card::Card {
            name: "test".to_owned(),
            health: rand::thread_rng().gen_range(1, 10),
            attack: rand::thread_rng().gen_range(1, 10),
            level: 1,
            exp: 0,
            durability: 10,
            class: "this".to_owned(),
            abilities: Vec::new(),
        };
        t.push(x)
    }
    let mut d = Deck::Deck { cards: t };

    // Grant cards a few levels 
    // This part will grant levels 1-5 to some of your cards untill you
    // run out of experience to give
    while exp_granting > 0 {
        //Determine which card gets booseted
        let mut which_card = 1;
        if num_cards > 1 {
            which_card = rand::thread_rng().gen_range(1, d.cards.len());
        }
        //figure out how many levels to give it
        let mut to_give = 0;
        if exp_granting > 600 {
            match rand::thread_rng().gen_range(1,5) {
                1 => to_give = 100,
                2 => to_give = 225,
                3 => to_give = 350,
                4 => to_give = 475,
                5 => to_give = 600,
                _ => {},
            };
            d.cards[which_card].give_exp(to_give)
        }
        else {
            to_give = exp_granting;
        }
        exp_granting = exp_granting - to_give;
    }


    d.print();
    d.save_to_file();




}

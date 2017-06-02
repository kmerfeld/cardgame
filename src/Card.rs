
#[derive(Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub abilities: Vec<String>,
    pub health: i32,
    pub attack: i32,
    pub level: i32,
    pub exp: i32,
    pub durability: i32,
    pub class: String,
}

impl Card {
    //Grant exp to a card
    pub fn give_exp(&mut self, x: i32) {
        self.exp = &self.exp + x;
        if self.exp > (self.level * 125 + 100 ) {
            self.level = &self.level + 1;
            // Here we roll on the table based on the cards class
        }
    }


    pub fn pretty_print(&self) {
        println!("name:\t{}\nclass:\t{}\nhealth:\t{}\nattack:\t{}\nlevel:\t{}\nexp:\t{}\ndura:\t{}",
                 &self.name,
                 &self.class,
                 &self.health,
                 &self.attack,
                 &self.level,
                 &self.exp,
                 &self.durability);
        for i in &self.abilities {
            println!("{}", i);

        }
    }
}

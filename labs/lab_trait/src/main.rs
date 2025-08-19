struct Player {
    name: String,
    level: u8,
    atk: u16,
    def: u16
}

trait PlayerBehavior {
    fn display(&self);
}

impl PlayerBehavior for Player {
    fn display(&self) {
        println!("Player Information:\nName: {:?}\nLevel: {:?}\nAttack: {:?}\nDefence: {:?}", self.name, self.level, self.atk, self.def)
    }
}

fn main() {
    let hero = Player {
        name: String::from("Yoka Zaka"),
        level: 100,
        atk: 2600,
        def: 1800
    };
    hero.display();
}

#[derive(Debug)]
pub struct Animal {
    pub leg_cnt: u64,
}

impl Default for Animal {
    fn default() -> Self {
        Self { leg_cnt: 4 }
    }
}

impl Animal {
    pub fn print_leg_cnt(&self) {
        println!("Leg cnt: {}", self.leg_cnt);
    }
}

pub fn run_default_trait() {
    let animal = Animal::default();
    animal.print_leg_cnt();
}

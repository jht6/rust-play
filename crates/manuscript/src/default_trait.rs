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
    // 直接调用struct的trait方法
    let animal = Animal::default();
    animal.print_leg_cnt();

    // 根据变量类型让Default::default()自动创建值
    let animal2: Animal = Default::default();
    animal2.print_leg_cnt();
}

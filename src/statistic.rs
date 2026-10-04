#[derive(Debug)]
pub struct Stat {
    pub max_hp: u32,
    pub hp: u32,
    pub max_mana: u32,
    pub mana: u32,
}

impl Stat {
    pub fn new() -> Self {
        Stat {
            max_hp: 100,
            hp: 100,
            max_mana: 100,
            mana: 100,
        }
    }
}

pub const MAX_LEVEL: u32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Bronze,
    Gold,
    Platinum,
    Hero,
}

impl Rank {
    pub fn from_level(level: u32) -> Rank {
        match level {
            0..=24 => Rank::Bronze,
            25..=49 => Rank::Gold,
            50..=99 => Rank::Platinum,
            _ => Rank::Hero,
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub level: u32,
    pub exp: u32,
    pub gold: u32,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            level: 1,
            exp: 0,
            gold: 0,
        }
    }

    pub fn exp_to_next(&self) -> u32 {
        100 + (self.level - 1) * 50
    }

    pub fn rank(&self) -> Rank {
        Rank::from_level(self.level)
    }

    pub fn gain_exp(&mut self, amount: u32) -> u32 {
        if self.level == MAX_LEVEL {
            return 0;
        }

        self.exp += amount;
        let mut level_gained = 0;

        while self.level < MAX_LEVEL && self.exp >= self.exp_to_next() {
            self.exp -= self.exp_to_next();
            self.level += 1;
            level_gained = 1;
        }

        if self.level >= MAX_LEVEL {
            self.exp = 0;
        }

        level_gained
    }
}

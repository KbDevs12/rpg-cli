use crate::inventory::Inventory;

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
    pub inventory: Inventory,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            level: 1,
            exp: 0,
            gold: 0,
            inventory: Inventory::new(),
        }
    }

    pub fn exp_to_next(&self) -> u32 {
        100 + (self.level - 1) * 50
    }

    pub fn rank(&self) -> Rank {
        Rank::from_level(self.level)
    }

    pub fn gain_exp(&mut self, amount: u32) -> u32 {
        if self.level >= MAX_LEVEL {
            return 0;
        }

        self.exp += amount;
        let mut level_gained = 0;

        while self.level < MAX_LEVEL && self.exp >= self.exp_to_next() {
            self.exp -= self.exp_to_next();
            self.level += 1;
            level_gained += 1;
        }

        if self.level >= MAX_LEVEL {
            self.exp = 0;
        }

        level_gained
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naik_level_satu_kali() {
        let mut p = Player::new("Budi");
        assert_eq!(p.gain_exp(100), 1);
        assert_eq!(p.level, 2);
        assert_eq!(p.exp, 0);
    }

    #[test]
    fn naik_level_sekaligus() {
        let mut p = Player::new("Budi");
        // 1 ke 2 kan butuh 100 exp berarti ke 3 itu butuh 150 totalnya jadi 250
        assert_eq!(p.gain_exp(260), 2);
        assert_eq!(p.level, 3);
        assert_eq!(p.exp, 10);
    }

    #[test]
    fn level_tidak_lebih_dari_100() {
        let mut p = Player::new("Budi");
        p.gain_exp(u32::MAX / 2);
        assert_eq!(p.level, MAX_LEVEL);
        assert_eq!(p.rank(), Rank::Hero);
    }

    #[test]
    fn rank_sesuai_level() {
        assert_eq!(Rank::from_level(1), Rank::Bronze);
        assert_eq!(Rank::from_level(25), Rank::Gold);
        assert_eq!(Rank::from_level(50), Rank::Platinum);
        assert_eq!(Rank::from_level(100), Rank::Hero);
    }
}

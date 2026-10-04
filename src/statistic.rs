use crate::player::MAX_LEVEL;
use std::fmt;

#[derive(Debug)]
pub struct Stat {
    pub max_hp: u32,
    pub hp: u32,
    pub max_mana: u32,
    pub mana: u32,
    pub attack: u32,
    pub defense: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StatisticError {
    HpAlreadyFull,
    ManaAlreadyFull,
}

impl fmt::Display for StatisticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::HpAlreadyFull => write!(f, "Hp lu udah full!"),
            Self::ManaAlreadyFull => write!(f, "Mana lu udah full nih!"),
        }
    }
}

// persiapan aja pake mana soalnya kepikiran buat add staff
impl Stat {
    pub fn new() -> Self {
        Stat {
            max_hp: 100,
            hp: 100,
            max_mana: 100,
            mana: 100,
            attack: 10,
            defense: 5,
        }
    }

    pub fn level_up(&mut self, level: u32) {
        let level = level.clamp(1, MAX_LEVEL);
        self.max_hp = 100 + (level - 1) * 10;
        self.max_mana = 100 + (level - 1) * 10;
        self.attack = 10 + (level - 1) * 2;
        self.defense = 5 + (level - 1);

        self.hp = self.max_hp;
        self.mana = self.max_mana;
    }

    pub fn get_damage(&mut self, damage: u32) {
        self.hp = self.hp.saturating_sub(damage);
    }

    pub fn heal(&mut self, healed: u32) -> Result<(), StatisticError> {
        if self.hp == self.max_hp {
            return Err(StatisticError::HpAlreadyFull);
        }

        self.hp = self.hp.saturating_add(healed).min(self.max_hp);

        Ok(())
    }

    pub fn restore_mana(&mut self, amount: u32) -> Result<(), StatisticError> {
        if self.mana == self.max_mana {
            return Err(StatisticError::ManaAlreadyFull);
        }
        self.mana = self.mana.saturating_add(amount).min(self.max_mana);
        Ok(())
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
}

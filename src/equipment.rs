use crate::inventory::{Gear, Item};

pub const FIST_NAME: &str = "Fist";
pub const FIST_POWER: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slot {
    Weapon,
    Shield,
}

impl Gear {
    pub fn slot(&self) -> Slot {
        match self {
            Gear::Weapon(_) => Slot::Weapon,
            Gear::Shield(_) => Slot::Shield,
        }
    }
}

#[derive(Debug, Default)]
pub struct Equipment {
    pub weapon: Option<Item>,
    pub shield: Option<Item>,
}

impl Equipment {
    pub fn slot_mut(&mut self, slot: Slot) -> &mut Option<Item> {
        match slot {
            Slot::Weapon => &mut self.weapon,
            Slot::Shield => &mut self.shield,
        }
    }

    pub fn weapon_name(&self) -> &'static str {
        self.weapon.map(|i| i.name()).unwrap_or(FIST_NAME)
    }

    pub fn attack_bonus(&self) -> u32 {
        match self.weapon.and_then(|i| i.gear()) {
            Some(Gear::Weapon(n)) => n,
            _ => FIST_POWER,
        }
    }

    pub fn defense_bonus(&self) -> u32 {
        match self.weapon.and_then(|i| i.gear()) {
            Some(Gear::Shield(n)) => n,
            _ => 1,
        }
    }
}

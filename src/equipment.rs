use crate::inventory::{Gear, Item};

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

#[derive(Debug)]
pub struct Equipment {
    pub weapon: Item,
    pub shield: Option<Item>,
}

impl Default for Equipment {
    fn default() -> Self {
        Equipment {
            weapon: Item::Fist,
            shield: None,
        }
    }
}

impl Equipment {
    /// Pasang item. Return item lama yang harus balik ke inventory.
    /// Fist tidak pernah dikembalikan karena bukan barang yang bisa disimpan.
    pub fn put_on(&mut self, item: Item, slot: Slot) -> Option<Item> {
        match slot {
            Slot::Weapon => {
                let old = std::mem::replace(&mut self.weapon, item);
                (old != Item::Fist).then_some(old)
            }
            Slot::Shield => self.shield.replace(item),
        }
    }

    /// Lepas slot. None kalau kosong / senjata masih Fist.
    pub fn take_off(&mut self, slot: Slot) -> Option<Item> {
        match slot {
            Slot::Weapon if self.weapon == Item::Fist => None,
            Slot::Weapon => Some(std::mem::replace(&mut self.weapon, Item::Fist)),
            Slot::Shield => self.shield.take(),
        }
    }

    pub fn weapon_name(&self) -> &'static str {
        self.weapon.name()
    }

    pub fn attack_bonus(&self) -> u32 {
        match self.weapon.gear() {
            Some(Gear::Weapon(n)) => n,
            _ => 0,
        }
    }

    pub fn defense_bonus(&self) -> u32 {
        match self.shield.and_then(|i| i.gear()) {
            Some(Gear::Shield(n)) => n,
            _ => 0,
        }
    }
}

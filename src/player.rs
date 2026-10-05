use crate::{
    equipment::{Equipment, Slot},
    inventory::{Effect, Inventory, Item},
    statistic::Stat,
};

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
    pub inventory: Inventory,
    pub statistic: Stat,
    pub equipment: Equipment,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            level: 1,
            exp: 0,
            inventory: Inventory::new(),
            statistic: Stat::new(),
            equipment: Equipment::default(),
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
            self.statistic.level_up(self.level);
        }

        if self.level >= MAX_LEVEL {
            self.exp = 0;
        }

        level_gained
    }

    pub fn use_item(&mut self, item: Item, qty: u32) -> Result<String, String> {
        let effect = item
            .effect()
            .ok_or_else(|| format!("{} tidak bisa dipakai", item.name()))?;

        let have = self.inventory.count(item);
        if qty == 0 || have < qty {
            return Err(format!("{} kurang. butuh {qty} buat dipakai", item.name()));
        }

        let msg = match effect {
            Effect::Heal(n) => {
                self.statistic
                    .heal(n.saturating_mul(qty))
                    .map_err(|e| e.to_string())?;
                format!(
                    "HP pulih, sekarang {}/{}",
                    self.statistic.hp, self.statistic.max_hp
                )
            }
            Effect::RestoreMana(n) => {
                self.statistic
                    .restore_mana(n.saturating_mul(qty))
                    .map_err(|e| e.to_string())?;
                format!(
                    "Mana pulih, sekarang {}/{}",
                    self.statistic.mana, self.statistic.max_mana
                )
            }
        };
        self.inventory
            .remove_item(item, qty)
            .map_err(|e| e.to_string())?;

        Ok(msg)
    }

    pub fn total_attack(&self) -> u32 {
        self.statistic.attack + self.equipment.attack_bonus()
    }

    pub fn total_defense(&self) -> u32 {
        self.statistic.defense + self.equipment.defense_bonus()
    }

    pub fn equip(&mut self, item: Item) -> Result<String, String> {
        let gear = item
            .gear()
            .ok_or_else(|| format!("{} ga bisa dipasang sebagai equipment", item.name()))?;

        self.inventory
            .remove_item(item, 1)
            .map_err(|e| e.to_string())?;

        if let Some(old) = self.equipment.put_on(item, gear.slot()) {
            self.inventory.add_item(old, 1);
        }

        Ok(format!("{} dipasang.", item.name()))
    }

    pub fn unequip(&mut self, slot: Slot) -> Result<String, String> {
        let item = self
            .equipment
            .take_off(slot)
            .ok_or_else(|| "Ga ada yang bisa dilepas".to_string())?;
        self.inventory.add_item(item, 1);
        Ok(format!("{} dilepas.", item.name()))
    }
}

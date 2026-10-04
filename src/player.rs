use crate::{
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
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            level: 1,
            exp: 0,
            inventory: Inventory::new(),
            statistic: Stat::new(),
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
        println!("{}", p.statistic.hp);
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

    #[test]
    fn potion_dipakai_hp_naik_item_berkurang() {
        let mut p = Player::new("Budi");
        p.inventory.add_item(Item::HealthPotion, 2);
        p.statistic.get_damage(60);
        assert!(p.use_item(Item::HealthPotion, 1).is_ok());
        assert_eq!(p.statistic.hp, 90);
        assert_eq!(p.inventory.count(Item::HealthPotion), 1);
    }

    #[test]
    fn potion_tidak_kepakai_kalau_hp_full() {
        let mut p = Player::new("Budi");
        p.inventory.add_item(Item::HealthPotion, 1);
        assert!(p.use_item(Item::HealthPotion, 1).is_err());
        assert_eq!(p.inventory.count(Item::HealthPotion), 1);
    }

    #[test]
    fn pakai_item_yang_tidak_dimiliki() {
        let mut p = Player::new("Budi");
        assert!(p.use_item(Item::HealthPotion, 1).is_err());
    }

    #[test]
    fn item_non_usable_ditolak() {
        let mut p = Player::new("Budi");
        p.inventory.add_item(Item::IronSword, 1);
        assert!(p.use_item(Item::IronSword, 1).is_err());
        assert_eq!(p.inventory.count(Item::IronSword), 1);
    }
}

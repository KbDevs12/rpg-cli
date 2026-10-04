use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    HealthPotion,
    ManaPotion,
    IronSword,
    WoodenShield,
}

impl Item {
    pub fn name(&self) -> &'static str {
        match self {
            Item::HealthPotion => "Health Potion",
            Item::ManaPotion => "Mana Potion",
            Item::IronSword => "Iron Sword",
            Item::WoodenShield => "Iron Shield",
        }
    }

    pub fn price(&self) -> u32 {
        match self {
            Item::HealthPotion => 50,
            Item::ManaPotion => 60,
            Item::IronSword => 300,
            Item::WoodenShield => 150,
        }
    }

    pub fn sell_price(&self) -> u32 {
        self.price() / 2
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InventoryError {
    NotEnoughGold { needed: u32, have: u32 },
    NotEnoughItem { item: Item, needed: u32, have: u32 },
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InventoryError::NotEnoughGold { needed, have } => {
                write!(
                    f,
                    "Uang kurang nih! butuh {needed}, sedangkan lu punya {have}"
                )
            }

            InventoryError::NotEnoughItem { item, needed, have } => {
                write!(
                    f,
                    "{} kurang bro, butuh {needed} padahal lu cuma ada  {have}",
                    item.name()
                )
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Inventory {
    gold: u32,
    items: HashMap<Item, u32>,
}

impl Inventory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn gold(&self) -> u32 {
        self.gold
    }

    pub fn add_gold(&mut self, amount: u32) {
        self.gold = self.gold.saturating_add(amount);
    }

    pub fn spend_gold(&mut self, amount: u32) -> Result<(), InventoryError> {
        if self.gold < amount {
            return Err(InventoryError::NotEnoughGold {
                needed: amount,
                have: self.gold,
            });
        }
        self.gold -= amount;
        Ok(())
    }

    pub fn count(&self, item: Item) -> u32 {
        self.items.get(&item).copied().unwrap_or(0)
    }

    pub fn add_item(&mut self, item: Item, qty: u32) {
        *self.items.entry(item).or_insert(0) += qty
    }

    pub fn remove_item(&mut self, item: Item, qty: u32) -> Result<(), InventoryError> {
        let have = self.count(item);

        if have < qty {
            return Err(InventoryError::NotEnoughItem {
                item,
                needed: qty,
                have,
            });
        }

        let left = have - qty;
        if left == 0 {
            self.items.remove(&item);
        } else {
            self.items.insert(item, left);
        }

        Ok(())
    }

    pub fn buy(&mut self, item: Item, qty: u32) -> Result<(), InventoryError> {
        self.spend_gold(item.price().saturating_mul(qty))?;
        self.add_item(item, qty);
        Ok(())
    }

    pub fn sell(&mut self, item: Item, qty: u32) -> Result<(), InventoryError> {
        self.remove_item(item, qty)?;
        self.add_gold(item.sell_price() * qty);
        Ok(())
    }

    pub fn list(&self) -> Vec<(Item, u32)> {
        let mut v: Vec<(Item, u32)> = self.items.iter().map(|(i, q)| (*i, *q)).collect();
        v.sort_by_key(|(item, _)| item.name());
        v
    }

    pub fn display(&self) {
        println!("=== Inventory ===");
        println!("Gold: {}", self.gold);
        if self.items.is_empty() {
            println!("(tidak ada item)");
        }
        for (item, qty) in self.list() {
            println!("- {} x{}", item.name(), qty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tambah_dan_kurang_gold() {
        let mut inv = Inventory::new();

        inv.add_gold(100);
        assert!(inv.spend_gold(40).is_ok());
        assert_eq!(inv.gold(), 60);

        assert_eq!(
            inv.spend_gold(100),
            Err(InventoryError::NotEnoughGold {
                needed: 100,
                have: 60
            })
        );
        // cek kalo gagal seharusnnya ga ngurang
        assert_eq!(inv.gold(), 60);
    }

    #[test]
    fn numpuk_item() {
        let mut inv = Inventory::new();
        inv.add_item(Item::HealthPotion, 2);
        inv.add_item(Item::HealthPotion, 3);
        assert_eq!(inv.count(Item::HealthPotion), 5);
    }

    #[test]
    fn abisin_item() {
        let mut inv = Inventory::new();
        inv.add_item(Item::IronSword, 1);
        assert!(inv.remove_item(Item::IronSword, 1).is_ok());
        assert_eq!(inv.count(Item::IronSword), 0);
        assert!(inv.list().is_empty());
        assert!(inv.remove_item(Item::IronSword, 1).is_err());
    }

    #[test]
    fn beli_item() {
        let mut inv = Inventory::new();
        inv.add_gold(120);
        assert!(inv.buy(Item::HealthPotion, 2).is_ok());
        assert_eq!(inv.gold(), 20);
        assert_eq!(inv.count(Item::HealthPotion), 2);
        assert!(inv.buy(Item::IronSword, 1).is_err());
        assert_eq!(inv.gold(), 20);
    }

    #[test]
    fn jual_item() {
        let mut inv = Inventory::new();
        inv.add_item(Item::IronSword, 1);
        assert!(inv.sell(Item::IronSword, 1).is_ok());
        assert_eq!(inv.gold(), 150);
        assert_eq!(inv.count(Item::IronSword), 0);
    }
}

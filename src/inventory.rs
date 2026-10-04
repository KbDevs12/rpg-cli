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

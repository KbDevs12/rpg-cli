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
}

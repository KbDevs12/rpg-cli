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

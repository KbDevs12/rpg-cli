#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonsterKind {
    Slime,
    Goblin,
    Orc,
    Dragon,
}

impl MonsterKind {
    pub fn candidates(level: u32) -> &'static [MonsterKind] {
        match level {
            0..=9 => &[MonsterKind::Slime, MonsterKind::Goblin],
            10..=29 => &[MonsterKind::Goblin, MonsterKind::Orc],
            _ => &[MonsterKind::Dragon],
        }
    }
}

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

#[derive(Debug)]
pub struct Monster {
    pub name: &'static str,
    pub hp: u32,
    pub max_hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub exp_reward: u32,
    pub gold_reward: u32,
}

impl Monster {
    pub fn new(kind: MonsterKind) -> Self {
        let (name, hp, attack, defense, exp, gold) = match kind {
            MonsterKind::Slime => ("Slime", 30, 8, 2, 40, 10),
            MonsterKind::Goblin => ("Goblin", 50, 12, 4, 60, 20),
            MonsterKind::Orc => ("Orc", 120, 22, 10, 150, 60),
            MonsterKind::Dragon => ("Dragon", 400, 45, 25, 600, 300),
        };
        Monster {
            name,
            hp,
            max_hp: hp,
            attack,
            defense,
            exp_reward: exp,
            gold_reward: gold,
        }
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.hp = self.hp.saturating_sub(damage);
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
}

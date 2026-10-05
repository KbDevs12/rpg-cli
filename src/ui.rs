use std::io::{self, Write};

use crate::inventory::Inventory;
use crate::monster::Monster;
use crate::player::{MAX_LEVEL, Player};

pub fn read_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn boxed(title: &str, lines: &[String]) {
    let width = lines
        .iter()
        .map(|l| l.chars().count())
        .chain(std::iter::once(title.chars().count()))
        .max()
        .unwrap_or(0);
    let inner = width + 2;

    println!("\n+{}+", "=".repeat(inner));
    println!("| {:^width$} |", title);
    println!("+{}+", "-".repeat(inner));
    for l in lines {
        println!("| {:<width$} |", l);
    }
    println!("+{}+", "=".repeat(inner));
}

pub fn bar(cur: u32, max: u32, len: usize) -> String {
    let filled = if max == 0 {
        0
    } else {
        (cur as u64 * len as u64 / max as u64) as usize
    };
    let filled = filled.min(len);
    format!("[{}{}]", "#".repeat(filled), "-".repeat(len - filled))
}

pub fn show_title() {
    println!(
        r"
  ____  ____   ____      ____ _     ___
 |  _ \|  _ \ / ___|    / ___| |   |_ _|
 | |_) | |_) | |  _    | |   | |    | |
 |  _ <|  __/| |_| |   | |___| |___ | |
 |_| \_\_|    \____|    \____|_____|___|
"
    );
}

pub fn show_menu(p: &Player) {
    boxed(
        &format!(
            "{}  Lv {}  {}",
            p.name,
            p.level,
            format!("{:?}", p.rank()).to_uppercase()
        ),
        &[
            format!(
                "HP {} {}/{}",
                bar(p.statistic.hp, p.statistic.max_hp, 15),
                p.statistic.hp,
                p.statistic.max_hp
            ),
            format!("Gold: {}", p.inventory.gold()),
            String::new(),
            "1. Berburu      4. Equipment".to_string(),
            "2. Profil       5. Toko".to_string(),
            "3. Inventory    6. Simpan & keluar".to_string(),
        ],
    );
}

pub fn show_profile(p: &Player) {
    let s = &p.statistic;
    let exp_line = if p.level >= MAX_LEVEL {
        "EXP    MAX LEVEL".to_string()
    } else {
        format!(
            "EXP    {} {}/{}",
            bar(p.exp, p.exp_to_next(), 20),
            p.exp,
            p.exp_to_next()
        )
    };
    boxed(
        "PROFIL",
        &[
            format!("Nama   : {}", p.name),
            format!("Level  : {}", p.level),
            format!("Rank   : {:?}", p.rank()),
            exp_line,
            String::new(),
            format!("HP     {} {}/{}", bar(s.hp, s.max_hp, 20), s.hp, s.max_hp),
            format!(
                "Mana   {} {}/{}",
                bar(s.mana, s.max_mana, 20),
                s.mana,
                s.max_mana
            ),
            String::new(),
            format!(
                "Attack : {} (base {} + {})",
                p.total_attack(),
                s.attack,
                p.equipment.attack_bonus()
            ),
            format!(
                "Defense: {} (base {} + {})",
                p.total_defense(),
                s.defense,
                p.equipment.defense_bonus()
            ),
            format!("Senjata: {}", p.equipment.weapon_name()),
            format!(
                "Shield : {}",
                p.equipment.shield.map(|i| i.name()).unwrap_or("-")
            ),
            format!("Gold   : {}", p.inventory.gold()),
        ],
    );
}

pub fn show_inventory(inv: &Inventory) {
    let mut lines = vec![format!("Gold: {}", inv.gold()), String::new()];
    let items = inv.list();
    if items.is_empty() {
        lines.push("(tidak ada item)".to_string());
    }
    for (item, qty) in items {
        lines.push(format!("{:<16} x{}", item.name(), qty));
    }
    boxed("INVENTORY", &lines);
}

pub fn battle_status(p: &Player, m: &Monster) {
    boxed(
        &format!("BATTLE vs {}", m.name),
        &[
            format!(
                "{:<8} {} {}/{}",
                m.name,
                bar(m.hp, m.max_hp, 20),
                m.hp,
                m.max_hp
            ),
            format!(
                "{:<8} {} {}/{}",
                "Kamu",
                bar(p.statistic.hp, p.statistic.max_hp, 20),
                p.statistic.hp,
                p.statistic.max_hp
            ),
        ],
    );
}

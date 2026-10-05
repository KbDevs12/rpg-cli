use rand::RngExt;

use crate::inventory::Item;
use crate::monster::Monster;
use crate::player::Player;
use crate::ui;

pub const MAX_ROLL_PCT: i32 = 10;

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Won {
        exp: u32,
        gold: u32,
        levels_gained: u32,
    },
    Lost,
    Fled,
}

pub fn calc_damage(attack: u32, defense: u32, roll_pct: i32) -> u32 {
    let factor = (100 + roll_pct).max(0) as u32;
    let boosted = attack.saturating_mul(factor) / 100;
    boosted.saturating_sub(defense).max(1)
}

pub fn player_attack(player: &Player, monster: &mut Monster, roll_pct: i32) -> u32 {
    let dmg = calc_damage(player.total_attack(), monster.defense, roll_pct);
    monster.take_damage(dmg);
    dmg
}

pub fn monster_attack(monster: &Monster, player: &mut Player, roll_pct: i32) -> u32 {
    let dmg = calc_damage(monster.attack, player.total_defense(), roll_pct);
    player.statistic.get_damage(dmg);
    dmg
}

pub fn try_flee(roll: u32) -> bool {
    roll < 50
}

pub fn give_reward(player: &mut Player, monster: &Monster) -> u32 {
    player.inventory.add_gold(monster.gold_reward);
    player.gain_exp(monster.exp_reward)
}

pub fn apply_defeat(player: &mut Player) {
    let loss = player.inventory.gold() / 2;
    let _ = player.inventory.spend_gold(loss);
    player.statistic.hp = player.statistic.max_hp;
}

pub fn run(player: &mut Player, mut monster: Monster) -> Outcome {
    let mut rng = rand::rng();
    println!("\n*** Muncul {}! ***", monster.name);

    loop {
        ui::battle_status(player, &monster);
        println!(
            "1. Serang   2. Health Potion (x{})   3. Kabur",
            player.inventory.count(Item::HealthPotion)
        );

        match ui::read_input().as_str() {
            "1" => {
                let roll = rng.random_range(-MAX_ROLL_PCT..=MAX_ROLL_PCT);
                let dmg = player_attack(player, &mut monster, roll);
                println!(
                    "Kamu menyerang pakai {}, {dmg} damage!",
                    player.equipment.weapon_name()
                );
            }
            "2" => match player.use_item(Item::HealthPotion, 1) {
                Ok(msg) => println!("{msg}"),
                Err(e) => {
                    println!("{e}");
                    continue; // gagal = tidak memakan giliran
                }
            },
            "3" => {
                if try_flee(rng.random_range(0..100)) {
                    println!("Berhasil kabur!");
                    return Outcome::Fled;
                }
                println!("Gagal kabur!");
            }
            _ => {
                println!("Pilihan tidak valid.");
                continue;
            }
        }

        if !monster.is_alive() {
            let (exp, gold) = (monster.exp_reward, monster.gold_reward);
            let levels_gained = give_reward(player, &monster);
            println!("{} kalah! +{exp} exp, +{gold} gold", monster.name);
            if levels_gained > 0 {
                println!("*** LEVEL UP! Sekarang level {} ***", player.level);
            }
            return Outcome::Won {
                exp,
                gold,
                levels_gained,
            };
        }

        let roll = rng.random_range(-MAX_ROLL_PCT..=MAX_ROLL_PCT);
        let dmg = monster_attack(&monster, player, roll);
        println!("{} menyerang, {dmg} damage!", monster.name);

        if !player.statistic.is_alive() {
            apply_defeat(player);
            println!("Kamu kalah... gold berkurang setengah.");
            return Outcome::Lost;
        }
    }
}

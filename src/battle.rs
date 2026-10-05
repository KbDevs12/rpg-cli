use rand::RngExt;
use std::io::{self, Write};

use crate::inventory::Item;
use crate::monster::Monster;
use crate::player::Player;

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

pub fn calc_damage(attack: u32, defense: u32, roll: u32) -> u32 {
    (attack + roll).saturating_sub(defense).max(1)
}

pub fn player_attack(player: &Player, monster: &mut Monster, roll: u32) -> u32 {
    let dmg = calc_damage(player.statistic.attack, monster.defense, roll);
    monster.take_damage(dmg);
    dmg
}

pub fn monster_attack(monster: &Monster, player: &mut Player, roll: u32) -> u32 {
    let dmg = calc_damage(monster.attack, player.statistic.defense, roll);
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

fn read_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn run(player: &mut Player, mut monster: Monster) -> Outcome {
    let mut rng = rand::rng();
    println!("\nMuncul {}!", monster.name);

    loop {
        println!(
            "\n{} HP {}/{} | Kamu HP {}/{}",
            monster.name, monster.hp, monster.max_hp, player.statistic.hp, player.statistic.max_hp
        );
        println!("1. Serang  2. Health Potion  3. Kabur");

        match read_input().as_str() {
            "1" => {
                let dmg = player_attack(player, &mut monster, rng.random_range(0..=4));
                println!("Kamu menyerang, {dmg} damage!");
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
                println!("LEVEL UP! Sekarang level {}", player.level);
            }
            return Outcome::Won {
                exp,
                gold,
                levels_gained,
            };
        }

        let dmg = monster_attack(&monster, player, rng.random_range(0..=4));
        println!("{} menyerang, {dmg} damage!", monster.name);

        if !player.statistic.is_alive() {
            apply_defeat(player);
            println!("Kamu kalah... gold berkurang setengah.");
            return Outcome::Lost;
        }
    }
}

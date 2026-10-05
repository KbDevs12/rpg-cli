#[cfg(test)]
mod tests {
    use rpg_cli::{battle::*, inventory::Item, monster::*, player::*};

    #[test]
    fn damage_minimal_satu() {
        assert_eq!(calc_damage(5, 100, 0), 1);
    }

    #[test]
    fn roll_persen() {
        assert_eq!(calc_damage(100, 0, 10), 110);
        assert_eq!(calc_damage(100, 0, -10), 90);
        assert_eq!(calc_damage(10, 4, 0), 6);
    }

    #[test]
    fn player_serang_monster_pakai_fist() {
        let p = Player::new("Budi");
        let mut m = Monster::new(MonsterKind::Slime); // def 2, hp 30
        let dmg = player_attack(&p, &mut m, 0); // (10 + 1) - 2
        assert_eq!(dmg, 9);
        assert_eq!(m.hp, 21);
    }

    #[test]
    fn pedang_plus_roll() {
        let mut p = Player::new("Budi");
        p.inventory.add_item(Item::IronSword, 1);
        p.equip(Item::IronSword).unwrap();
        let mut m = Monster::new(MonsterKind::Slime);
        assert_eq!(player_attack(&p, &mut m, 0), 18); // 20 - 2
        assert_eq!(player_attack(&p, &mut m, 10), 20); // 22 - 2
    }

    #[test]
    fn monster_serang_player() {
        let mut p = Player::new("Budi"); // def 5, hp 100
        let m = Monster::new(MonsterKind::Goblin); // atk 12
        let dmg = monster_attack(&m, &mut p, 0); // 12 - 5
        assert_eq!(dmg, 7);
        assert_eq!(p.statistic.hp, 93);
    }

    #[test]
    fn shield_mengurangi_damage() {
        let mut p = Player::new("Budi");
        p.inventory.add_item(Item::WoodenShield, 1);
        p.equip(Item::WoodenShield).unwrap();
        let m = Monster::new(MonsterKind::Goblin);
        assert_eq!(monster_attack(&m, &mut p, 0), 2); // 12 - (5 + 5)
    }

    #[test]
    fn hadiah_kemenangan() {
        let mut p = Player::new("Budi");
        let m = Monster::new(MonsterKind::Goblin); // exp 60, gold 20
        let levels = give_reward(&mut p, &m);
        assert_eq!(levels, 0);
        assert_eq!(p.exp, 60);
        assert_eq!(p.inventory.gold(), 20);
    }

    #[test]
    fn kalah_gold_berkurang_setengah_hp_pulih() {
        let mut p = Player::new("Budi");
        p.inventory.add_gold(101);
        p.statistic.get_damage(100);
        apply_defeat(&mut p);
        assert_eq!(p.inventory.gold(), 51); // 101 - 50
        assert_eq!(p.statistic.hp, p.statistic.max_hp);
    }
}

#[cfg(test)]
mod tests {
    use rpg_cli::{battle::*, monster::*, player::*};

    #[test]
    fn damage_minimal_satu() {
        assert_eq!(calc_damage(5, 100, 0), 1);
    }

    #[test]
    fn damage_normal() {
        assert_eq!(calc_damage(10, 4, 2), 8);
    }

    #[test]
    fn player_serang_monster() {
        let p = Player::new("Budi");
        let mut m = Monster::new(MonsterKind::Slime); // def 2, hp 30
        let dmg = player_attack(&p, &mut m, 0); // 10 - 2 = 8
        assert_eq!(dmg, 8);
        assert_eq!(m.hp, 22);
    }

    #[test]
    fn monster_serang_player() {
        let mut p = Player::new("Budi"); // def 5, hp 100
        let m = Monster::new(MonsterKind::Goblin); // atk 12
        let dmg = monster_attack(&m, &mut p, 0); // 12 - 5 = 7
        assert_eq!(dmg, 7);
        assert_eq!(p.statistic.hp, 93);
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

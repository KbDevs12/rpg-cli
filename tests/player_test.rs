#[cfg(test)]
mod tests {
    use rpg_cli::{inventory::*, player::*};

    #[test]
    fn naik_level_satu_kali() {
        let mut p = Player::new("Budi");
        assert_eq!(p.gain_exp(100), 1);
        assert_eq!(p.level, 2);
        assert_eq!(p.exp, 0);
        println!("{}", p.statistic.hp);
    }

    #[test]
    fn naik_level_sekaligus() {
        let mut p = Player::new("Budi");
        // 1 ke 2 kan butuh 100 exp berarti ke 3 itu butuh 150 totalnya jadi 250
        assert_eq!(p.gain_exp(260), 2);
        assert_eq!(p.level, 3);
        assert_eq!(p.exp, 10);
    }

    #[test]
    fn level_tidak_lebih_dari_100() {
        let mut p = Player::new("Budi");
        p.gain_exp(u32::MAX / 2);
        assert_eq!(p.level, MAX_LEVEL);
        assert_eq!(p.rank(), Rank::Hero);
    }

    #[test]
    fn rank_sesuai_level() {
        assert_eq!(Rank::from_level(1), Rank::Bronze);
        assert_eq!(Rank::from_level(25), Rank::Gold);
        assert_eq!(Rank::from_level(50), Rank::Platinum);
        assert_eq!(Rank::from_level(100), Rank::Hero);
    }

    #[test]
    fn potion_dipakai_hp_naik_item_berkurang() {
        let mut p = Player::new("Budi");
        p.inventory.add_item(Item::HealthPotion, 2);
        p.statistic.get_damage(60);
        assert!(p.use_item(Item::HealthPotion, 1).is_ok());
        assert_eq!(p.statistic.hp, 90);
        assert_eq!(p.inventory.count(Item::HealthPotion), 1);
    }

    #[test]
    fn potion_tidak_kepakai_kalau_hp_full() {
        let mut p = Player::new("Budi");
        p.inventory.add_item(Item::HealthPotion, 1);
        assert!(p.use_item(Item::HealthPotion, 1).is_err());
        assert_eq!(p.inventory.count(Item::HealthPotion), 1);
    }

    #[test]
    fn pakai_item_yang_tidak_dimiliki() {
        let mut p = Player::new("Budi");
        assert!(p.use_item(Item::HealthPotion, 1).is_err());
    }

    #[test]
    fn item_non_usable_ditolak() {
        let mut p = Player::new("Budi");
        p.inventory.add_item(Item::IronSword, 1);
        assert!(p.use_item(Item::IronSword, 1).is_err());
        assert_eq!(p.inventory.count(Item::IronSword), 1);
    }
}

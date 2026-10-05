#[cfg(test)]
mod tests {
    use rpg_cli::{db::Db, inventory::Item, player::Player};

    #[test]
    fn load_player_yang_tidak_ada() {
        let db = Db::open_in_memory().unwrap();
        assert!(db.load_player("Hantu").unwrap().is_none());
    }

    #[test]
    fn save_lalu_load_datanya_sama() {
        let mut db = Db::open_in_memory().unwrap();
        let mut p = Player::new("Budi");
        p.gain_exp(260); // level 3, exp 10
        p.inventory.add_gold(75);
        p.inventory.add_item(Item::HealthPotion, 3);
        p.statistic.get_damage(30);

        db.save_player(&p).unwrap();
        let loaded = db.load_player("Budi").unwrap().unwrap();

        assert_eq!(loaded.level, 3);
        assert_eq!(loaded.exp, 10);
        assert_eq!(loaded.inventory.gold(), 75);
        assert_eq!(loaded.inventory.count(Item::HealthPotion), 3);
        assert_eq!(loaded.statistic.hp, p.statistic.hp);
        assert_eq!(loaded.statistic.max_hp, p.statistic.max_hp);
        assert_eq!(loaded.statistic.attack, p.statistic.attack);
    }

    #[test]
    fn save_dua_kali_meng_update_bukan_menggandakan() {
        let mut db = Db::open_in_memory().unwrap();
        let mut p = Player::new("Budi");
        p.inventory.add_item(Item::HealthPotion, 2);
        db.save_player(&p).unwrap();
        p.gain_exp(100);
        db.save_player(&p).unwrap();

        let loaded = db.load_player("Budi").unwrap().unwrap();
        assert_eq!(loaded.level, 2);
        assert_eq!(loaded.inventory.count(Item::HealthPotion), 2);
    }

    #[test]
    fn item_yang_habis_hilang_dari_db() {
        let mut db = Db::open_in_memory().unwrap();
        let mut p = Player::new("Budi");
        p.inventory.add_item(Item::Bread, 2);
        db.save_player(&p).unwrap();

        p.inventory.remove_item(Item::Bread, 2).unwrap();
        db.save_player(&p).unwrap();

        let loaded = db.load_player("Budi").unwrap().unwrap();
        assert_eq!(loaded.inventory.count(Item::Bread), 0);
    }

    #[test]
    fn player_baru_senjatanya_fist() {
        let mut db = Db::open_in_memory().unwrap();
        db.save_player(&Player::new("Budi")).unwrap();
        let loaded = db.load_player("Budi").unwrap().unwrap();
        assert_eq!(loaded.equipment.weapon, Item::Fist);
        assert_eq!(loaded.equipment.shield, None);
    }

    #[test]
    fn equipment_tersimpan_dan_kembali() {
        let mut db = Db::open_in_memory().unwrap();
        let mut p = Player::new("Budi");
        p.inventory.add_item(Item::IronSword, 1);
        p.inventory.add_item(Item::WoodenShield, 1);
        p.equip(Item::IronSword).unwrap();
        p.equip(Item::WoodenShield).unwrap();
        db.save_player(&p).unwrap();

        let loaded = db.load_player("Budi").unwrap().unwrap();
        assert_eq!(loaded.equipment.weapon, Item::IronSword);
        assert_eq!(loaded.equipment.shield, Some(Item::WoodenShield));
        assert_eq!(loaded.inventory.count(Item::IronSword), 0); // ga dobel
        assert_eq!(loaded.total_attack(), p.total_attack());
    }
}

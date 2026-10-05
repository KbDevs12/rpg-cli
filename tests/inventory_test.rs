#[cfg(test)]
mod tests {
    use rpg_cli::inventory::*;

    #[test]
    fn tambah_dan_kurang_gold() {
        let mut inv = Inventory::new();

        inv.add_gold(100);
        assert!(inv.spend_gold(40).is_ok());
        assert_eq!(inv.gold(), 60);

        assert_eq!(
            inv.spend_gold(100),
            Err(InventoryError::NotEnoughGold {
                needed: 100,
                have: 60
            })
        );
        // cek kalo gagal seharusnnya ga ngurang
        assert_eq!(inv.gold(), 60);
    }

    #[test]
    fn numpuk_item() {
        let mut inv = Inventory::new();
        inv.add_item(Item::HealthPotion, 2);
        inv.add_item(Item::HealthPotion, 3);
        assert_eq!(inv.count(Item::HealthPotion), 5);
    }

    #[test]
    fn abisin_item() {
        let mut inv = Inventory::new();
        inv.add_item(Item::IronSword, 1);
        assert!(inv.remove_item(Item::IronSword, 1).is_ok());
        assert_eq!(inv.count(Item::IronSword), 0);
        assert!(inv.list().is_empty());
        assert!(inv.remove_item(Item::IronSword, 1).is_err());
    }

    #[test]
    fn beli_item() {
        let mut inv = Inventory::new();
        inv.add_gold(120);
        assert!(inv.buy(Item::HealthPotion, 2).is_ok());
        assert_eq!(inv.gold(), 20);
        assert_eq!(inv.count(Item::HealthPotion), 2);
        assert!(inv.buy(Item::IronSword, 1).is_err());
        assert_eq!(inv.gold(), 20);
    }

    #[test]
    fn jual_item() {
        let mut inv = Inventory::new();
        inv.add_item(Item::IronSword, 1);
        assert!(inv.sell(Item::IronSword, 1).is_ok());
        assert_eq!(inv.gold(), 150);
        assert_eq!(inv.count(Item::IronSword), 0);
    }

    #[test]
    fn fist_ga_bisa_dibeli_atau_dijual() {
        let mut inv = Inventory::new();
        inv.add_gold(1000);
        assert_eq!(
            inv.buy(Item::Fist, 1),
            Err(InventoryError::NotForSale { item: Item::Fist })
        );
        assert_eq!(
            inv.sell(Item::Fist, 1),
            Err(InventoryError::NotForSale { item: Item::Fist })
        );
        assert_eq!(inv.gold(), 1000);
    }

    #[test]
    fn add_item_nol_ga_bikin_entri() {
        let mut inv = Inventory::new();
        inv.add_item(Item::Bread, 0);
        assert!(inv.list().is_empty());
    }
}

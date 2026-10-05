use rand::RngExt;
use rpg_cli::{
    battle,
    db::Db,
    equipment::Slot,
    inventory::Item,
    monster::{Monster, MonsterKind},
    player::Player,
    ui,
};

fn main() {
    let mut db = Db::open("rpg.db").expect("gagal membuka database");

    ui::show_title();
    println!("Nama karakter:");
    let name = ui::read_input();
    if name.is_empty() {
        println!("Nama ga boleh kosong.");
        return;
    }

    let mut player = match db.load_player(&name) {
        Ok(Some(p)) => {
            println!("Selamat datang kembali, {}!", p.name);
            p
        }
        Ok(None) => {
            println!("Karakter baru dibuat: {name}");
            let mut p = Player::new(&name);
            p.inventory.add_item(Item::HealthPotion, 3);
            p
        }
        Err(e) => {
            println!("Gagal load data: {e}");
            return;
        }
    };

    let mut rng = rand::rng();

    loop {
        ui::show_menu(&player);
        match ui::read_input().as_str() {
            "1" => {
                let list = MonsterKind::candidates(player.level);
                let kind = list[rng.random_range(0..list.len())];
                battle::run(&mut player, Monster::new(kind));
                save(&mut db, &player); // autosave setelah battle
            }
            "2" => ui::show_profile(&player),
            "3" => inventory_menu(&mut player, &mut db),
            "4" => equipment_menu(&mut player, &mut db),
            "5" => shop_menu(&mut player, &mut db),
            "6" => {
                save(&mut db, &player);
                println!("Data tersimpan. Sampai jumpa!");
                break;
            }
            _ => println!("Pilihan tidak valid."),
        }
    }
}

fn save(db: &mut Db, player: &Player) {
    if let Err(e) = db.save_player(player) {
        println!("Gagal menyimpan: {e}");
    }
}

fn pick(len: usize) -> Option<usize> {
    ui::read_input()
        .parse::<usize>()
        .ok()
        .filter(|n| (1..=len).contains(n))
        .map(|n| n - 1)
}

fn read_qty() -> Option<u32> {
    println!("Jumlah:");
    ui::read_input().parse::<u32>().ok().filter(|&n| n > 0)
}

fn inventory_menu(player: &mut Player, db: &mut Db) {
    ui::show_inventory(&player.inventory);

    let usable: Vec<Item> = player
        .inventory
        .list()
        .into_iter()
        .map(|(i, _)| i)
        .filter(|i| i.is_usable())
        .collect();
    if usable.is_empty() {
        return;
    }

    println!("Pakai item? (nomor, 0 = kembali)");
    for (i, item) in usable.iter().enumerate() {
        println!("{}. {}", i + 1, item.name());
    }
    let Some(i) = pick(usable.len()) else {
        return;
    };

    match player.use_item(usable[i], 1) {
        Ok(msg) => {
            println!("{msg}");
            save(db, player);
        }
        Err(e) => println!("{e}"),
    }
}

fn equipment_menu(player: &mut Player, db: &mut Db) {
    ui::boxed(
        "EQUIPMENT",
        &[
            format!(
                "Senjata : {} (+{} atk)",
                player.equipment.weapon_name(),
                player.equipment.attack_bonus()
            ),
            format!(
                "Shield  : {} (+{} def)",
                player.equipment.shield.map(|i| i.name()).unwrap_or("-"),
                player.equipment.defense_bonus()
            ),
            String::new(),
            format!(
                "Total ATK {} | DEF {}",
                player.total_attack(),
                player.total_defense()
            ),
            String::new(),
            "1. Pasang  2. Lepas senjata  3. Lepas shield  0. Kembali".to_string(),
        ],
    );

    let result = match ui::read_input().as_str() {
        "1" => {
            let gear: Vec<Item> = player
                .inventory
                .list()
                .into_iter()
                .map(|(i, _)| i)
                .filter(|i| i.gear().is_some())
                .collect();
            if gear.is_empty() {
                println!("Ga ada equipment di inventory.");
                return;
            }
            for (i, item) in gear.iter().enumerate() {
                println!("{}. {}", i + 1, item.name());
            }
            match pick(gear.len()) {
                Some(i) => player.equip(gear[i]),
                None => Err("Pilihan tidak valid.".to_string()),
            }
        }
        "2" => player.unequip(Slot::Weapon),
        "3" => player.unequip(Slot::Shield),
        _ => return,
    };

    match result {
        Ok(msg) => {
            println!("{msg}");
            save(db, player);
        }
        Err(e) => println!("{e}"),
    }
}

fn shop_menu(player: &mut Player, db: &mut Db) {
    let stock: Vec<Item> = Item::ALL
        .iter()
        .copied()
        .filter(|i| i.is_purchasable())
        .collect();

    loop {
        let mut lines = vec![
            format!("Gold kamu: {}", player.inventory.gold()),
            String::new(),
        ];
        for (i, item) in stock.iter().enumerate() {
            lines.push(format!(
                "{}. {:<14} {:>5} g  (jual {})",
                i + 1,
                item.name(),
                item.price(),
                item.sell_price()
            ));
        }
        lines.push(String::new());
        lines.push("b. Beli   s. Jual   0. Kembali".to_string());
        ui::boxed("TOKO", &lines);

        match ui::read_input().as_str() {
            "b" => {
                println!("Nomor item:");
                let Some(i) = pick(stock.len()) else {
                    println!("Pilihan tidak valid.");
                    continue;
                };
                let Some(qty) = read_qty() else {
                    println!("Jumlah tidak valid.");
                    continue;
                };
                match player.inventory.buy(stock[i], qty) {
                    Ok(()) => {
                        println!("Berhasil beli {} x{}", stock[i].name(), qty);
                        save(db, player);
                    }
                    Err(e) => println!("{e}"),
                }
            }
            "s" => {
                let bag: Vec<Item> = player
                    .inventory
                    .list()
                    .into_iter()
                    .map(|(i, _)| i)
                    .filter(|i| i.is_purchasable())
                    .collect();
                if bag.is_empty() {
                    println!("Ga ada yang bisa dijual.");
                    continue;
                }
                for (i, item) in bag.iter().enumerate() {
                    println!(
                        "{}. {} x{} (jual {} g)",
                        i + 1,
                        item.name(),
                        player.inventory.count(*item),
                        item.sell_price()
                    );
                }
                let Some(i) = pick(bag.len()) else {
                    println!("Pilihan tidak valid.");
                    continue;
                };
                let Some(qty) = read_qty() else {
                    println!("Jumlah tidak valid.");
                    continue;
                };
                match player.inventory.sell(bag[i], qty) {
                    Ok(()) => {
                        println!("Berhasil jual {} x{}", bag[i].name(), qty);
                        save(db, player);
                    }
                    Err(e) => println!("{e}"),
                }
            }
            "0" => break,
            _ => println!("Pilihan tidak valid."),
        }
    }
}

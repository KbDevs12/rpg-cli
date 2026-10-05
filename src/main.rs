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

const MAX_NAME_LEN: usize = 16;

enum Next {
    Switch,
    Quit,
}

fn main() {
    let mut db = Db::open("rpg.db").expect("gagal membuka database");

    while let Some(mut player) = select_player(&mut db) {
        match game_loop(&mut player, &mut db) {
            Next::Switch => {}
            Next::Quit => break,
        }
    }
    println!("Sampai jumpa!");
}

fn save(db: &mut Db, player: &Player) {
    if let Err(e) = db.save_player(player) {
        println!("Gagal menyimpan: {e}");
    }
}

fn select_player(db: &mut Db) -> Option<Player> {
    loop {
        ui::clear();
        ui::show_title();

        let saved = match db.list_players() {
            Ok(v) => v,
            Err(e) => {
                println!("Gagal baca data: {e}");
                return None;
            }
        };

        let mut lines: Vec<String> = saved
            .iter()
            .enumerate()
            .map(|(i, (name, level))| format!("{}. {} (Lv {})", i + 1, name, level))
            .collect();
        if saved.is_empty() {
            lines.push("(belum ada karakter)".to_string());
        }
        lines.push(String::new());
        lines.push("n. Karakter baru   q. Keluar".to_string());
        ui::boxed("PILIH KARAKTER", &lines);

        let input = ui::read_input();
        match input.as_str() {
            "q" => return None,
            "n" => {
                if let Some(p) = create_player(db) {
                    return Some(p);
                }
            }
            other => {
                let choice = other
                    .parse::<usize>()
                    .ok()
                    .filter(|n| (1..=saved.len()).contains(n));
                let Some(n) = choice else {
                    println!("Pilihan tidak valid.");
                    ui::pause();
                    continue;
                };

                match db.load_player(&saved[n - 1].0) {
                    Ok(Some(p)) => return Some(p),
                    Ok(None) => {
                        println!("Data karakter ga ketemu.");
                        ui::pause();
                    }
                    Err(e) => {
                        println!("Gagal load data: {e}");
                        ui::pause();
                    }
                }
            }
        }
    }
}

fn create_player(db: &mut Db) -> Option<Player> {
    println!("Nama karakter baru (kosong = batal):");
    let name = ui::read_input();
    if name.is_empty() {
        return None;
    }
    if name.chars().count() > MAX_NAME_LEN {
        println!("Nama maksimal {MAX_NAME_LEN} karakter.");
        ui::pause();
        return None;
    }

    match db.load_player(&name) {
        Ok(Some(_)) => {
            println!("Nama '{name}' sudah dipakai karakter lain.");
            ui::pause();
            return None;
        }
        Ok(None) => {}
        Err(e) => {
            println!("Gagal cek nama: {e}");
            ui::pause();
            return None;
        }
    }

    let mut p = Player::new(&name);
    p.inventory.add_item(Item::HealthPotion, 3);
    save(db, &p);
    Some(p)
}

fn game_loop(player: &mut Player, db: &mut Db) -> Next {
    let mut rng = rand::rng();

    loop {
        ui::clear();
        ui::show_menu(player);

        match ui::read_input().as_str() {
            "1" => {
                let list = MonsterKind::candidates(player.level);
                let kind = list[rng.random_range(0..list.len())];
                battle::run(player, Monster::new(kind));
                save(db, player); // autosave setelah battle
                ui::pause();
            }
            "2" => {
                ui::show_profile(player);
                ui::pause();
            }
            "3" => inventory_menu(player, db),
            "4" => equipment_menu(player, db),
            "5" => shop_menu(player, db),
            "6" => {
                save(db, player);
                return Next::Switch;
            }
            "7" => {
                save(db, player);
                println!("Data tersimpan.");
                return Next::Quit;
            }
            _ => {
                println!("Pilihan tidak valid.");
                ui::pause();
            }
        }
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
    ui::clear();
    ui::show_inventory(&player.inventory);

    let usable: Vec<Item> = player
        .inventory
        .list()
        .into_iter()
        .map(|(i, _)| i)
        .filter(|i| i.is_usable())
        .collect();
    if usable.is_empty() {
        ui::pause();
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
    ui::pause();
}

fn equipment_menu(player: &mut Player, db: &mut Db) {
    ui::clear();
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
                ui::pause();
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
    ui::pause();
}

fn shop_menu(player: &mut Player, db: &mut Db) {
    let stock: Vec<Item> = Item::ALL
        .iter()
        .copied()
        .filter(|i| i.is_purchasable())
        .collect();
    let mut notice = String::new();

    loop {
        ui::clear();
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

        if !notice.is_empty() {
            println!("{notice}");
            notice.clear();
        }

        match ui::read_input().as_str() {
            "b" => {
                println!("Nomor item:");
                let Some(i) = pick(stock.len()) else {
                    notice = "Pilihan tidak valid.".to_string();
                    continue;
                };
                let Some(qty) = read_qty() else {
                    notice = "Jumlah tidak valid.".to_string();
                    continue;
                };
                match player.inventory.buy(stock[i], qty) {
                    Ok(()) => {
                        notice = format!("Berhasil beli {} x{}", stock[i].name(), qty);
                        save(db, player);
                    }
                    Err(e) => notice = e.to_string(),
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
                    notice = "Ga ada yang bisa dijual.".to_string();
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
                    notice = "Pilihan tidak valid.".to_string();
                    continue;
                };
                let Some(qty) = read_qty() else {
                    notice = "Jumlah tidak valid.".to_string();
                    continue;
                };
                match player.inventory.sell(bag[i], qty) {
                    Ok(()) => {
                        notice = format!("Berhasil jual {} x{}", bag[i].name(), qty);
                        save(db, player);
                    }
                    Err(e) => notice = e.to_string(),
                }
            }
            "0" => break,
            _ => notice = "Pilihan tidak valid.".to_string(),
        }
    }
}

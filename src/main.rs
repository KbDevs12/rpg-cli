use rpg_cli::{
    battle,
    inventory::Item,
    monster::{Monster, MonsterKind},
    player::Player,
};

fn main() {
    let mut p = Player::new("Adit");
    let naik = p.gain_exp(120);

    println!("Kenalin gua {}.", p.name);
    println!("{:?} | naik {} level | rank: {:?}", p, naik, p.rank());

    p.inventory.add_gold(500);

    match p.inventory.buy(Item::HealthPotion, 3) {
        Ok(()) => println!("Pembelian berhasil!"),
        Err(e) => println!("Gagal: {e}"),
    }

    if let Err(e) = p.inventory.buy(Item::IronSword, 5) {
        println!("Gagal: {e}");
    }

    p.inventory.display();

    let kind = MonsterKind::candidates(p.level)[0];
    battle::run(&mut p, Monster::new(kind));
    p.inventory.display();
}

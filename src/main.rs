mod player;

use player::Player;
fn main() {
    let mut p = Player::new("Adit");
    let naik = p.gain_exp(120);
    println!("Kenalin gua {}.", p.name);
    println!("{:?} | naik {} level | rank: {:?}", p, naik, p.rank());
}

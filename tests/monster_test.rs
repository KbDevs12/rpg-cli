#[cfg(test)]
mod test {
    use rpg_cli::monster::*;

    #[test]
    fn damage_tidak_bikin_hp_negatif() {
        let mut m = Monster::new(MonsterKind::Slime);
        m.take_damage(9999);
        assert_eq!(m.hp, 0);
        assert!(!m.is_alive());
    }
}

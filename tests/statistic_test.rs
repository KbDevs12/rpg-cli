#[cfg(test)]
mod tests {
    use rpg_cli::statistic::*;

    #[test]
    fn naik_level() {
        let mut stat = Stat::new();
        stat.level_up(2);

        assert_eq!(stat.max_hp, 110);
    }

    #[test]
    fn kena_damage_terus_heal() {
        let mut stat = Stat::new();
        stat.get_damage(20);
        println!("hp: {}", stat.hp);

        assert_eq!(stat.hp, 80);
        // harusnya mau nge heal berapapun gabakal lebihin max_hp
        assert!(stat.heal(500).is_ok());
        assert_eq!(stat.hp, stat.max_hp);
    }

    #[test]
    fn heal_saat_full_ditolak() {
        let mut stat = Stat::new();
        assert_eq!(stat.heal(10), Err(StatisticError::HpAlreadyFull));
    }
}

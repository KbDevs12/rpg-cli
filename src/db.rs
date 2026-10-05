use rusqlite::{Connection, OptionalExtension, Result, params};

use crate::inventory::{Gear, Item};
use crate::player::{MAX_LEVEL, Player};

pub struct Db {
    conn: Connection,
}

const SCHEMA: &str = "
    PRAGMA foreign_keys = ON;

    CREATE TABLE IF NOT EXISTS players (
        id     INTEGER PRIMARY KEY AUTOINCREMENT,
        name   TEXT    NOT NULL UNIQUE,
        level  INTEGER NOT NULL,
        exp    INTEGER NOT NULL,
        gold   INTEGER NOT NULL,
        hp     INTEGER NOT NULL,
        mana   INTEGER NOT NULL,
        weapon TEXT    NOT NULL DEFAULT 'fist',
        shield TEXT
    );

    CREATE TABLE IF NOT EXISTS inventory (
        player_id INTEGER NOT NULL REFERENCES players(id) ON DELETE CASCADE,
        item_key  TEXT    NOT NULL,
        qty       INTEGER NOT NULL CHECK (qty > 0),
        PRIMARY KEY (player_id, item_key)
    );
";

struct PlayerRow {
    id: i64,
    level: u32,
    exp: u32,
    gold: u32,
    hp: u32,
    mana: u32,
    weapon: String,
    shield: Option<String>,
}

impl Db {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(SCHEMA)?;
        Ok(Db { conn })
    }

    // buat testing aja naro database di ram
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(SCHEMA)?;
        Ok(Db { conn })
    }

    pub fn save_player(&mut self, p: &Player) -> Result<()> {
        let tx = self.conn.transaction()?;

        tx.execute(
            "INSERT INTO players (name, level, exp, gold, hp, mana, weapon, shield)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(name) DO UPDATE SET
                level  = excluded.level,
                exp    = excluded.exp,
                gold   = excluded.gold,
                hp     = excluded.hp,
                mana   = excluded.mana,
                weapon = excluded.weapon,
                shield = excluded.shield",
            params![
                p.name,
                p.level,
                p.exp,
                p.inventory.gold(),
                p.statistic.hp,
                p.statistic.mana,
                p.equipment.weapon.key(),
                p.equipment.shield.map(|i| i.key()),
            ],
        )?;

        let id: i64 = tx.query_row(
            "SELECT id FROM players WHERE name = ?1",
            params![p.name],
            |row| row.get(0),
        )?;

        tx.execute("DELETE FROM inventory WHERE player_id = ?1", params![id])?;
        for (item, qty) in p.inventory.list() {
            tx.execute(
                "INSERT INTO inventory (player_id, item_key, qty) VALUES (?1, ?2, ?3)",
                params![id, item.key(), qty],
            )?;
        }

        tx.commit()
    }

    pub fn load_player(&self, name: &str) -> Result<Option<Player>> {
        let row = self
            .conn
            .query_row(
                "SELECT id, level, exp, gold, hp, mana, weapon, shield
                 FROM players WHERE name = ?1",
                params![name],
                |r| {
                    Ok(PlayerRow {
                        id: r.get(0)?,
                        level: r.get(1)?,
                        exp: r.get(2)?,
                        gold: r.get(3)?,
                        hp: r.get(4)?,
                        mana: r.get(5)?,
                        weapon: r.get(6)?,
                        shield: r.get(7)?,
                    })
                },
            )
            .optional()?;

        let Some(row) = row else {
            return Ok(None);
        };

        let level = row.level.clamp(1, MAX_LEVEL);

        let mut p = Player::new(name);
        p.level = level;
        p.exp = row.exp;
        p.statistic.level_up(level);
        p.statistic.hp = row.hp.min(p.statistic.max_hp);
        p.statistic.mana = row.mana.min(p.statistic.max_mana);
        p.inventory.add_gold(row.gold);

        p.equipment.weapon = Item::from_key(&row.weapon)
            .filter(|i| matches!(i.gear(), Some(Gear::Weapon(_))))
            .unwrap_or(Item::Fist);
        p.equipment.shield = row
            .shield
            .as_deref()
            .and_then(Item::from_key)
            .filter(|i| matches!(i.gear(), Some(Gear::Shield(_))));

        let mut stmt = self
            .conn
            .prepare("SELECT item_key, qty FROM inventory WHERE player_id = ?1")?;
        let rows = stmt.query_map(params![row.id], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, u32>(1)?))
        })?;

        for item_row in rows {
            let (key, qty) = item_row?;
            if let Some(item) = Item::from_key(&key) {
                p.inventory.add_item(item, qty);
            }
        }

        Ok(Some(p))
    }
}

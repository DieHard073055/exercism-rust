// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        let new_level = self.level;
        let new_mana;
        let new_health = 100u32;
        if self.health != 0 {
            return None;
        }
        match new_level {
            n if n > 9 => new_mana = Some(100),
            _ => new_mana = None
        }
        Some(Player {
            health: new_health,
            mana: new_mana,
            level: new_level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(n) if n < mana_cost => 0,
            None => {
                match self.health {
                    n if n < mana_cost => self.health = 0,
                    _ => self.health -= mana_cost,
                };
                0
            }
            Some(n) if n > mana_cost => {
                let old_mana = self.mana.unwrap();
                self.mana = Some(old_mana - mana_cost);
                2 * mana_cost
            }
            _ => unreachable!("unexpected amount of mana!"),
        }
    }
}

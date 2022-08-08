// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}
// here
impl Player {
    pub fn revive(&self) -> Option<Player> {
        match (&self.health == &0, &self.level >= &10) {
            (true, true ) => Some(Player {
                health: 100,
                mana: Some(100),
                level: self.level,
            }),
            (true, false) => Some(Player {
                health: 100,
                mana: None,
                level: self.level,
            }),
            (_, _) => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) if mana >= mana_cost => mana - mana_cost,
            Some(_) => 0,
            None => {
                self.health = self.health - if self.health < mana_cost { self.health } else { mana_cost };
                0
            }
        }
    }
}

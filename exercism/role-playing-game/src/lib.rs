// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        //unimplemented!("Revive this player")
        //check health =0, return player health = 100
        //if level also >= 10, mana=100, else mana = None
        // level same as before
        (self.health == 0).then(|| Player {
            health: 100,
            mana: self.level.ge(&10).then(|| 100),
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        //unimplemented!("Cast a spell of cost {}", mana_cost)
        //if player mana == none or 0, decrease health, damage return 0
        //if player does not have enough mana, return 0
        //deduct mana cost, return 2* mana_cost
        match self.mana {
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
            Some(mana) if mana < mana_cost => 0,
            Some(mana) if mana >= mana_cost => {
                self.mana = Some(self.mana.unwrap() - mana_cost);
                2 * mana_cost
            }
            Some(_) => unreachable!(),
        }
    }
}

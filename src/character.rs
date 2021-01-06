use rand::random;

use crate::cooking::shrimp_cooking_chance_for_level;
use crate::level::level_for_experience;

pub struct CookingStats {
    pub cooked_shrimp: usize,
    pub burnt_shrimp: usize,
}

pub struct Character {
    cooking_exp: usize,
    pub cooking_stats: CookingStats,
}

impl Character {
    pub fn after_tutorial_island() -> Character {
        Character {
            // one shrimp + one bread
            cooking_exp: 30 + 40,
            cooking_stats: CookingStats {
                cooked_shrimp: 1,
                burnt_shrimp: 0,
            },
        }
    }

    pub fn cook_one_shrimp(&mut self) {
        let level = level_for_experience(self.cooking_exp);
        let cook_chance = shrimp_cooking_chance_for_level(level);

        let r = random::<f32>();
        if r >= cook_chance {
            // burnt
            self.cooking_stats.burnt_shrimp += 1;
        } else {
            // cooked
            self.cooking_exp += 30;
            self.cooking_stats.cooked_shrimp += 1;
        }
    }

    pub fn cooking_level(&self) -> usize {
        level_for_experience(self.cooking_exp)
    }
}

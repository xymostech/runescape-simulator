fn experience_to_next_level(level: usize) -> f64 {
    (level as f64 + 300.0 * 2.0_f64.powf(level as f64 / 7.0)).floor() / 4.0
}

pub fn level_for_experience(exp: usize) -> usize {
    let mut remaining_exp: f64 = exp as f64;

    for level in 1..99 {
        if (experience_to_next_level(level) - remaining_exp).floor() > 0.0 {
            return level;
        } else {
            remaining_exp -= experience_to_next_level(level);
        }
    }

    99
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_for_experience_should_be_correct() {
        assert_eq!(level_for_experience(0), 1);
        assert_eq!(level_for_experience(82), 1);
        assert_eq!(level_for_experience(83), 2);
        assert_eq!(level_for_experience(101332), 49);
        assert_eq!(level_for_experience(101333), 50);
        assert_eq!(level_for_experience(13034430), 98);
        assert_eq!(level_for_experience(13034431), 99);
    }
}

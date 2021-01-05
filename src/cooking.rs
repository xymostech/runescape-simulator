pub fn shrimp_cooking_chance_for_level(level: usize) -> f32 {
    if level >= 34 {
        return 1.0;
    }

    // 50.39% at level 1, 99.22% at level 33
    (0.9922 - 0.5039) / (33.0 - 1.0) * (level - 1) as f32 + 0.5039
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shrimp_cooking_chances_should_be_correct() {
        assert_eq!(shrimp_cooking_chance_for_level(1), 0.5039);
        assert_eq!(shrimp_cooking_chance_for_level(33), 0.9922);
        assert_eq!(shrimp_cooking_chance_for_level(34), 1.0);
    }
}

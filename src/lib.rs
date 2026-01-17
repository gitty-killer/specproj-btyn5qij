pub fn score(value: i32) -> i32 {
    if value < 0 { 0 } else { value * 2 }
}

pub fn score_all(values: &[i32]) -> Vec<i32> {
    values.iter().map(|v| score(*v)).collect()
}

#[cfg(test)]
mod tests {
    use super::{score, score_all};

    #[test]
    fn score_basic() {
        assert_eq!(score(3), 6);
        assert_eq!(score(-1), 0);
    }

    #[test]
    fn score_list() {
        assert_eq!(score_all(&[1, 2]), vec![2, 4]);
    }
}

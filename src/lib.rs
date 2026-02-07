pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_one_and_one() {
        assert_eq!(add(1, 1), 2);
    }

    #[test]
    fn adds_negative_and_positive() {
        assert_eq!(add(-1, 2), 1);
    }
}

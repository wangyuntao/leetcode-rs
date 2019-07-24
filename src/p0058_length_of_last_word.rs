pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim().rsplit(char::is_whitespace).next().unwrap().len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_owned()), 5);
    }
}

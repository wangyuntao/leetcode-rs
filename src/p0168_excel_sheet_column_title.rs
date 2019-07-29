pub struct Solution;

impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        assert!(n > 0);
        let mut n = n;
        std::iter::from_fn(move || match n {
            0 => None,
            _ => {
                n -= 1;
                let d = n % 26 + 'A' as i32;
                n /= 26;
                Some(d as u8 as char)
            }
        })
        .collect::<Vec<char>>()
        .into_iter()
        .rev()
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::convert_to_title(1), "A");
        assert_eq!(Solution::convert_to_title(28), "AB");
        assert_eq!(Solution::convert_to_title(701), "ZY");
        assert_eq!(Solution::convert_to_title(704), "AAB");
    }
}

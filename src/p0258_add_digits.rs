pub struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut r = num;
        while r > 9 {
            let mut n = 0;
            while r > 0 {
                n += r % 10;
                r /= 10;
            }
            r = n;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::add_digits(38), 2);
    }
}

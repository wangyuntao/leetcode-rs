pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut v = digits;
        if v.iter_mut().rev().all(|x| match *x {
            9 => {
                *x = 0;
                true
            }
            _ if *x < 9 => {
                *x += 1;
                false
            }
            _ => panic!("illegal argument"),
        }) {
            v.insert(0, 1);
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }
}

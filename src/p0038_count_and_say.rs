pub struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        assert!(n >= 1);
        let mut r = "1".to_owned();
        let mut i = 1;

        while i < n {
            let mut ci = r.chars();
            let mut c1 = ci.nth(0).unwrap();
            let mut cn = 1;
            let mut r0 = String::new();

            ci.for_each(|c2| {
                if c2 == c1 {
                    cn += 1;
                } else {
                    r0.push_str(&cn.to_string());
                    r0.push(c1);
                    c1 = c2;
                    cn = 1;
                }
            });

            r0.push_str(&cn.to_string());
            r0.push(c1);
            r = r0;
            i += 1;
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::count_and_say(1), "1");
        assert_eq!(Solution::count_and_say(5), "111221");
    }
}

pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (mut a, b) = if a.len() > b.len() { (a, b) } else { (b, a) };
        let mut n = 0;
        let v = a
            .chars()
            .rev()
            .zip(b.chars().rev().chain(std::iter::repeat('0')))
            .map(
                |(ac, bc)| match ac.to_digit(2).unwrap() + bc.to_digit(2).unwrap() + n {
                    d @ 0..=3 => {
                        n = d / 2;
                        char::from((d % 2 + 48) as u8)
                    }
                    _ => panic!("illegal argument"),
                },
            )
            .collect::<Vec<char>>();

        a.clear();
        if n == 1 {
            a.push('1');
        }
        a.extend(v.into_iter().rev());
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::add_binary("1010".to_owned(), "1011".to_owned()),
            "10101"
        );
    }
}

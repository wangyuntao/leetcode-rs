#[derive(Default)]
pub struct MinStack {
    v: Vec<i32>,
    m: Option<i32>,
}

impl MinStack {
    /** initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, x: i32) {
        self.v.push(x);
        if let Some(m) = self.m {
            if x < m {
                self.m = Some(x);
            }
        }
    }

    pub fn pop(&mut self) {
        let x = self.v.pop().unwrap();
        if let Some(m) = self.m {
            if x == m {
                self.m = None;
            }
        }
    }

    pub fn top(&self) -> i32 {
        self.v[self.v.len() - 1]
    }

    pub fn get_min(&mut self) -> i32 {
        self.m.unwrap_or_else(|| {
            let m = self.v.iter().min().unwrap();
            let m = *m;
            self.m = Some(m);
            m
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut s = MinStack::new();
        s.push(-2);
        s.push(0);
        s.push(-3);
        assert_eq!(s.get_min(), -3);
        s.pop();
        assert_eq!(s.top(), 0);
        assert_eq!(s.get_min(), -2);
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { val, next }
    }
}

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head == None {
            return None;
        }

        let mut hd = head;
        let mut b1 = hd.as_mut().unwrap();

        while let Some(b2) = b1.next.as_mut() {
            if b2.val != b1.val {
                b1 = b1.next.as_mut().unwrap();
                continue;
            }
            let b2n = std::mem::replace(&mut b2.next, None);
            std::mem::replace(&mut b1.next, b2n);
        }

        hd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let l = Some(Box::new(ListNode::new(2, None)));
        let l = Some(Box::new(ListNode::new(1, l)));
        let l = Some(Box::new(ListNode::new(1, l)));
        let l = Solution::delete_duplicates(l);

        assert_eq!(l.as_ref().unwrap().val, 1);
        assert_eq!(l.as_ref().unwrap().next.as_ref().unwrap().val, 2);
        assert_eq!(l.unwrap().next.unwrap().next, None);
    }
}

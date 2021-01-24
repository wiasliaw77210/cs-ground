#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

#[derive(Debug)]
struct SinglyLinkedList {
    head: Option<Box<ListNode>>
}

impl SinglyLinkedList {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, val: i32) {
        let next = self.head.take();
        self.head = Some(Box::new(ListNode { val, next }));
    }
}

pub struct Solution;

impl Solution {
    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ptr = SinglyLinkedList::new();
        let mut v = Vec::new();
        let mut a = l1.as_deref().take();
        let mut b = l2.as_deref().take();
        let mut c: i32 = 0;
        loop {
            match (a, b) {
                (Some(v1), Some(v2)) => {
                    v.push((v1.val+v2.val+c)%10);
                    c = if (v1.val+v2.val+c)>=10 { 1 } else { 0 };
                },
                (None, Some(v2)) => {
                    v.push((v2.val+c)%10);
                    c = if (v2.val+c)>=10 { 1 } else { 0 };
                },
                (Some(v1), None) => {
                    v.push((v1.val+c)%10);
                    c = if (v1.val+c)>=10 { 1 } else { 0 };
                },
                (None, None) => {
                    if c == 1 {
                        v.push(c);
                    }
                    break ();
                }
            }
            a = match a.as_mut() {
                Some(node) => {
                    node.next.as_deref().take()
                },
                None => None
            };
            b = match b.as_mut() {
                Some(node) => {
                    node.next.as_deref().take()
                },
                None => None
            };
        }
        v.reverse();
        for i in v.iter() {
            ptr.push(*i);
        }
        return ptr.head;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let cases: [(i32, i32, i32); 3] = [(2, 5, 8), (4, 6, 0), (3, 4, 7)];
        let mut l1 = SinglyLinkedList::new();
        let mut l2 = SinglyLinkedList::new();
        let mut ans = SinglyLinkedList::new();
        cases.iter().for_each(|e| {
            l1.push(e.0);
            l2.push(e.1);
            ans.push(e.2);
        });
        assert_eq!(ans.head, Solution::add_two_numbers(l1.head, l2.head));
    }
}

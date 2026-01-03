// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = Self::length(&head);
        Self::merge_sort(head, len)
    }

    fn length(head: &Option<Box<ListNode>>) -> usize {
        let mut count = 0;
        let mut current = head.as_ref();
        while let Some(node) = current {
            count += 1;
            current = node.next.as_ref();
        }
        count
    }

    fn merge_sort(head: Option<Box<ListNode>>, len: usize) -> Option<Box<ListNode>> {
        if len <= 1 {
            return head;
        }
        let mid = len / 2;
        let (left, right) = Self::split_at(head, mid);
        let left_sorted = Self::merge_sort(left, mid);
        let right_sorted = Self::merge_sort(right, len-mid);
        Self::merge(left_sorted, right_sorted)
    }

    fn split_at(mut head: Option<Box<ListNode>>, mid: usize) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        if mid == 0 {
            return (None, head);
        }
        let mut cur = &mut head;
        for _ in 0..(mid-1) {
            cur = &mut cur.as_mut().unwrap().next;
        }
        let right = cur.as_mut().unwrap().next.take();
        (head, right)
    }

    fn merge(a: Option<Box<ListNode>>, b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut a = a;
        let mut b = b;

        let mut dummy = Box::new(ListNode::new(0));
        let mut tail: &mut Box<ListNode> = &mut dummy;

        while a.is_some() && b.is_some() {
            let take_a = a.as_ref().unwrap().val <= b.as_ref().unwrap().val;
            if take_a {
                let mut node = a.take().unwrap();
                a = node.next.take();
                tail.next = Some(node);
            } else {
                let mut node = b.take().unwrap();
                b = node.next.take();
                tail.next = Some(node);
            }
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if a.is_some() { a } else { b };
        dummy.next
    }
}
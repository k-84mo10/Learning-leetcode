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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return;
        }

        fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev = None;
            while let Some(mut node) = head {
                let next = node.next.take();
                node.next = prev;
                prev = Some(node);
                head = next;
            }
            prev
        }

        let mut len = 0;
        let mut p = head.as_ref();
        while let Some(node) = p {
            len += 1;
            p = node.next.as_ref();
        }

        let mid = (len - 1) / 2;

        let mut second = {
            let mut first_half = &mut *head;   
            for _ in 0..mid {
                first_half = &mut first_half.as_mut().unwrap().next;
            }

            let second = first_half.as_mut().unwrap().next.take();
            reverse(second)
        };

        let mut first = &mut *head;        
        while second.is_some() {
            let first_next = first.as_mut().unwrap().next.take();
            let second_next = second.as_mut().unwrap().next.take();

            first.as_mut().unwrap().next = second;
            first.as_mut().unwrap().next.as_mut().unwrap().next = first_next;

            first = &mut first
                .as_mut()
                .unwrap()
                .next
                .as_mut()
                .unwrap()
                .next;

            second = second_next;
        }
    }
}
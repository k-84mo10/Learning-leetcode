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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut previous = dummy.as_mut().unwrap();
        let mut digit = 0;
        let mut curr_l1 = l1.as_ref();
        let mut curr_l2 = l2.as_ref();

        while curr_l1.is_some() || curr_l2.is_some() || digit != 0{
            let l1_val = curr_l1.as_ref().map_or(0, |node| node.val);
            let l2_val = curr_l2.as_ref().map_or(0, |node| node.val);
            let sum = l1_val + l2_val + digit;
            digit = sum/10;
            previous.next = Some(Box::new(ListNode::new(sum%10)));
            previous = previous.next.as_mut().unwrap();

            curr_l1 = curr_l1.and_then(|node| node.next.as_ref());
            curr_l2 = curr_l2.and_then(|node| node.next.as_ref());
        }

        dummy.unwrap().next
    }
}
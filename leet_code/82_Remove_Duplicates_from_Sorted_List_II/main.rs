// Code needed to run locally
struct Solution;

fn main() {
    let tests = vec![vec![1, 2, 3, 3, 4, 4, 5], vec![1, 1, 1, 2, 3]];
    let expected = vec![vec![1, 2, 5], vec![2, 3]];
    for i in 0..tests.len() {
        let linked_list = vec_to_list(&tests[i]);
        let linked_list = Solution::delete_duplicates(linked_list);
        let deduped_list = list_to_vec(&linked_list);
        println!("{:?} == {:?}", expected[i], deduped_list);
        assert_eq!(expected[i], list_to_vec(&linked_list));
    }
}

fn vec_to_list(vec: &Vec<i32>) -> Option<Box<ListNode>> {
    if vec.len() == 0 {
        return None;
    }
    let mut head = Some(Box::new(ListNode::new(vec[0])));
    let mut curr = &mut head;
    for val in &vec[1..] {
        let new_node = Box::new(ListNode::new(*val));
        curr.as_mut().unwrap().next = Some(new_node);
        curr = &mut curr.as_mut().unwrap().next;
    }
    head
}
fn list_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut curr = head;

    while curr.is_some() {
        vec.push(curr.as_ref().unwrap().val);
        curr = &curr.as_ref().unwrap().next;
    }

    vec
}

// copied from leetcode
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// actual solution
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let prev_val = head.as_ref().unwrap().val - 1;
        let mut dummy = Some(Box::new(ListNode {
            next: head,
            val: prev_val,
        }));

        let node = &mut dummy.as_mut().unwrap().next;
        Solution::delete_duplicates_recursive(node, prev_val);
        dummy.unwrap().next
    }
    fn delete_duplicates_recursive(
        node: &mut Option<Box<ListNode>>,
        prev_val: i32,
    ) -> &Option<Box<ListNode>> {
        match node {
            None => &None,
            Some(n) if n.val == prev_val => {
                *node = n.next.take();
                Solution::delete_duplicates_recursive(node, prev_val)
            }
            Some(n) if n.next.is_some() && n.val == n.next.as_ref().unwrap().val => {
                let prev_val = n.val;
                Solution::delete_duplicates_recursive(node, prev_val)
            }
            Some(n) => Solution::delete_duplicates_recursive(&mut n.next, prev_val),
        }
    }
}

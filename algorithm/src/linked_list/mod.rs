// Definition for singly-linked list.
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

use std::{cell::RefCell, rc::Rc};
struct Solution {}
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre: Option<Box<ListNode>> = None;
        let mut head: Option<Box<ListNode>> = head;
        while let Some(mut n) = head {
            head = n.next;
            n.next = pre;
            pre = Some(n);
        }
        pre
    }

    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut res = ListNode::new(0);
        let head = res.clone();
        let mut lists = lists;

        let mut end = lists.len() - 1;

        for i in (0..end / 2 + 1).rev() {
            Self::rise(&mut lists, i, end)
        }

        loop {
            res.next = lists[0].clone();
            res = *res.next.unwrap();

            lists[0] = lists[0].clone().unwrap().next;

            let tmp = lists[0].clone();
            lists[0] = lists[end].clone();
            lists[end] = tmp;

            if let None = lists[end] {
                if end == 0 {
                    break;
                }
                end -= 1;
            }

            Self::rise(&mut lists, 0, end)
        }

        head.next
    }

    fn rise(lists: &mut Vec<Option<Box<ListNode>>>, k: usize, end: usize) {
        let mut k = k;
        loop {
            let mut i = 2 * k + 1;
            if i > end {
                break;
            }
            if i < end && lists[i + 1].as_ref().unwrap().val < lists[i].as_ref().unwrap().val {
                i += 1
            }
            if lists[k].as_ref().unwrap().val <= lists[i].as_ref().unwrap().val {
                break;
            }
            let tmp = lists[k].clone();
            lists[k] = lists[i].clone();
            lists[i] = tmp.clone();

            k = i;
        }
    }
}

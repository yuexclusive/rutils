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

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut head, mut pre) = (head, None);
        while let Some(mut n) = head {
            head = n.next;
            n.next = pre;
            pre = Some(n)
        }
        pre
    }
}

use std::borrow::BorrowMut;
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut list1, mut list2) = (list1, list2);
        let mut res = ListNode::new(0);
        let mut head = res.borrow_mut();

        while let (Some(v1), Some(v2)) = (&list1, &list2) {
            if v1.val > v2.val {
                head.next = Some(v2.clone());
                list2 = list2.unwrap().next;
            } else {
                head.next = Some(v1.clone());
                list1 = list1.unwrap().next;
            }
            head = head.next.as_deref_mut().unwrap();
        }

        head.next = list1.or(list2);
        res.next
    }
}

impl Solution {
    pub fn merge_k_lists_mine(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let lists = &mut lists
            .into_iter()
            .filter(|x| x.is_some())
            .collect::<Vec<Option<Box<ListNode>>>>();

        if lists.len() == 0 {
            return None;
        }
        let mut end = lists.len() - 1;
        (0..=end / 2).rev().for_each(|k| {
            Self::rise(lists, k, end);
        });

        let mut res = ListNode::new(0);
        let mut head = &mut res;
        loop {
            head.next = lists[0].clone();
            head = head.next.as_deref_mut().unwrap();
            lists[0] = lists[0].as_deref().unwrap().next.clone();
            if lists[0].is_none() {
                if end == 0 {
                    break;
                }
                lists.swap(0, end);
                end -= 1;
            }
            Self::rise(lists, 0, end);
        }
        res.next
    }

    fn rise(slice: &mut Vec<Option<Box<ListNode>>>, k: usize, end: usize) {
        let mut k = k;
        loop {
            let mut i = 2 * k + 1;
            if i > end {
                break;
            }
            if i < end && slice[i + 1].as_deref().unwrap().val < slice[i].as_deref().unwrap().val {
                i += 1
            }
            if slice[k].as_deref().unwrap().val < slice[i].as_deref().unwrap().val {
                break;
            }
            slice.swap(i, k);
            k = i;
        }
    }
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut res = ListNode::new(0);
        let mut head = &mut res;
        let mut heap = BinaryHeap::new();
        for mut item in lists {
            while let Some(node) = item {
                heap.push(Reverse(node.val));
                item = node.next;
            }
        }

        while let Some(Reverse(v)) = heap.pop() {
            head.next = Some(Box::new(ListNode::new(v)));
            head = head.next.as_deref_mut().unwrap();
        }

        res.next
    }
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head.clone();
        let mut total = 0;
        let mut vec = Vec::new();
        while let Some(v) = head {
            vec.push(v.val);
            head = v.next;
            total += 1;
        }

        let mut res = ListNode::new(0);
        let mut head = &mut res;

        for (_, v) in vec
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != total as usize - n as usize)
        {
            head.next = Some(Box::new(ListNode::new(*v)));
            head = head.next.as_deref_mut().unwrap();
        }

        res.next
    }
}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut vec = vec![];
        let mut h1 = &mut head.clone();
        while let Some(node) = h1 {
            vec.push(node.val);
            h1 = &mut node.next;
        }
        if vec.len() == 0 {
            return;
        }

        let mut vec2 = vec![];
        for i in 0..vec.len() / 2 {
            vec2.push(vec[i]);
            vec2.push(vec[vec.len() - i - 1]);
        }
        if vec.len() % 2 == 1 {
            vec2.push(vec[vec.len() / 2])
        }

        let mut i = 0;
        let mut h1 = head;
        while let Some(node) = h1 {
            node.val = vec2[i];
            i += 1;
            h1 = &mut node.next;
        }
    }
}

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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        let mut res = ListNode::new(0);
        let mut head = &mut res;
        for mut item in lists {
            while let Some(node) = item {
                heap.push(Reverse(node.val));
                item = node.next;
            }
        }

        while let Some(Reverse(val)) = heap.pop() {
            head.next = Some(Box::new(ListNode::new(val)));
            head = head.next.as_deref_mut().unwrap()
        }

        res.next
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        let mut hm = HashMap::new();
        let mut res = Vec::new();
        for v in nums {
            hm.entry(v).and_modify(|v| *v += 1).or_insert(1);
        }
        for (k, v) in hm {
            heap.push((v, k))
        }
        for _ in 0..k {
            if let Some((_, k)) = heap.pop() {
                res.push(k);
            }
        }

        res
    }
}

use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
struct MedianFinder {
    max: RefCell<BinaryHeap<i32>>,
    min: RefCell<BinaryHeap<Reverse<i32>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            max: RefCell::new(BinaryHeap::new()),
            min: RefCell::new(BinaryHeap::new()),
        }
    }

    fn add_num(&self, num: i32) {
        if self.max.borrow().len() == 0 {
            self.max.borrow_mut().push(num);
        } else {
            let v = self.max.borrow().peek().unwrap().clone();
            if num <= v {
                self.max.borrow_mut().push(num);
            } else {
                self.min.borrow_mut().push(Reverse(num));
            }
        }
        if self.max.borrow().len() < self.min.borrow().len() {
            self.max
                .borrow_mut()
                .push(self.min.borrow_mut().pop().unwrap().0)
        }
        if self.max.borrow().len() - self.min.borrow().len() == 2 {
            self.min
                .borrow_mut()
                .push(Reverse(self.max.borrow_mut().pop().unwrap()))
        }
    }

    fn find_median(&self) -> f64 {
        match self.max.borrow().len().cmp(&self.min.borrow().len()) {
            std::cmp::Ordering::Greater => self.max.borrow().peek().unwrap().clone() as f64,
            _ => {
                (self.max.borrow().peek().unwrap().clone() as f64
                    + self.min.borrow().peek().unwrap().0.clone() as f64)
                    / 2_f64
            }
        }
    }
}

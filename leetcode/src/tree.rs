// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let mut node = node.borrow_mut();
                let left = Self::max_depth(node.left.take());
                let right = Self::max_depth(node.right.take());
                left.max(right) + 1
            }
            None => 0,
        }
    }
}

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (_, None) | (None, _) => false,
            (Some(a), Some(b)) => {
                let (mut a, mut b) = (a.borrow_mut(), b.borrow_mut());
                a.val == b.val
                    && Self::is_same_tree(a.left.take(), b.left.take())
                    && Self::is_same_tree(a.right.take(), b.right.take())
            }
        }
    }
}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref node) = root {
            let mut n = node.borrow_mut();
            let l = n.left.take();
            n.left = Self::invert_tree(n.right.take());
            n.right = Self::invert_tree(l);
        }
        root
    }
}

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = i32::MIN;
        Self::max_node_sum(root, &mut res);
        res
    }

    fn max_node_sum(root: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
        match root {
            Some(node) => {
                let mut n = node.borrow_mut();
                let left = Self::max_node_sum(n.left.take(), res);
                let right = Self::max_node_sum(n.right.take(), res);
                *res = (*res).max(n.val + left.max(0) + right.max(0));
                n.val + left.max(right).max(0)
            }
            None => 0,
        }
    }
}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut level_index = 0;
        let mut current_level_count = 0;
        let mut current_level_capacity = 1;
        let mut next_level_capacity = 0;
        while let Some(n) = queue.pop_front() {
            current_level_count += 1;
            if let Some(n) = n {
                if res.len() <= level_index {
                    res.push(vec![]);
                }
                let mut n_ref = n.borrow_mut();
                res[level_index].push(n_ref.val);
                queue.push_back(n_ref.left.take());
                queue.push_back(n_ref.right.take());
                next_level_capacity += 2;
            }
            if current_level_count == current_level_capacity {
                current_level_capacity = next_level_capacity;
                current_level_count = 0;
                next_level_capacity = 0;
                level_index += 1;
            }
        }
        res
    }
}

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while let Some(node) = queue.pop_front() {
            if let Some(n) = node {
                let mut n = n.borrow_mut();
                res.push(n.val.to_string());
                queue.push_back(n.left.take());
                queue.push_back(n.right.take());
            } else {
                res.push("-".to_string());
            }
        }
        res.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let res: Vec<Option<Rc<RefCell<TreeNode>>>> = data
            .split(",")
            .map(|x| match x {
                "-" => None,
                x => Some(Rc::new(RefCell::new(TreeNode::new(x.parse().unwrap())))),
            })
            .collect();

        let (mut a, mut b, mut c) = (0, 1, 2);
        while c < res.len() {
            while res[a].is_none() {
                a += 1;
            }
            let mut node = res[a].as_ref().unwrap().borrow_mut();
            node.left = res[b].clone();
            node.right = res[c].clone();
            a += 1;
            b += 2;
            c += 2;
        }
        res[0].clone()
    }
}

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root, sub_root) {
            (None, None) => true,
            (None, _) | (_, None) => false,
            (Some(a), Some(b)) => {
                let a_ref = a.borrow();
                Self::is_same_node(&Some(a.clone()), &Some(b.clone()))
                    || Self::is_subtree(a_ref.left.clone(), Some(b.clone()))
                    || Self::is_subtree(a_ref.right.clone(), Some(b.clone()))
            }
        }
    }

    fn is_same_node(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (a, b) {
            (None, None) => true,
            (None, _) | (_, None) => false,
            (Some(a), Some(b)) => {
                let a = a.borrow();
                let b = b.borrow();
                a.val == b.val
                    && Self::is_same_node(&a.left, &b.left)
                    && Self::is_same_node(&a.right, &b.right)
            }
        }
    }
}

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let middle = inorder
            .iter()
            .enumerate()
            .find(|(_, v)| **v == preorder[0])
            .unwrap()
            .0;

        let mut res = TreeNode::new(preorder[0]);
        res.left = Self::build_tree(preorder[1..middle + 1].to_vec(), inorder[..middle].to_vec());
        res.right = Self::build_tree(
            preorder[middle + 1..].to_vec(),
            inorder[middle + 1..].to_vec(),
        );

        Some(Rc::new(RefCell::new(res)))
    }
}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut pre = None;
        let mut root = root;
        let mut stack = VecDeque::new();
        while !root.is_none() || !stack.is_empty() {
            while let Some(next) = root {
                stack.push_back(next.clone());
                root = next.borrow_mut().left.take();
            }
            let last = stack.pop_back();
            let v = last.as_ref().unwrap().borrow().val;
            if let Some(pre) = pre {
                if v <= pre {
                    return false;
                }
            }
            pre = Some(v);
            root = last.unwrap().borrow_mut().right.take();
        }
        true
    }
}

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = VecDeque::new();
        let mut root = root;
        let mut k = k;
        while root.is_some() || !stack.is_empty() {
            while let Some(r) = root {
                stack.push_back(r.clone());
                root = r.borrow_mut().left.take()
            }
            k -= 1;
            let last = stack.pop_back();
            if k == 0 {
                return last.as_deref().unwrap().borrow().val;
            }
            root = last.unwrap().borrow_mut().right.take()
        }
        0
    }
}

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (mut v1, mut v2) = (
            p.as_ref().unwrap().borrow().val,
            q.as_ref().unwrap().borrow().val,
        );
        if v1 > v2 {
            std::mem::swap(&mut v1, &mut v2)
        }
        let v = root.as_ref().unwrap().borrow().val;
        match (v.cmp(&v1), v.cmp(&v2)) {
            (std::cmp::Ordering::Less, _) => Self::lowest_common_ancestor(
                root.as_ref().unwrap().borrow_mut().right.take(),
                Some(Rc::new(RefCell::new(TreeNode::new(v1)))),
                Some(Rc::new(RefCell::new(TreeNode::new(v2)))),
            ),
            (_, std::cmp::Ordering::Greater) => Self::lowest_common_ancestor(
                root.as_ref().unwrap().borrow_mut().left.take(),
                Some(Rc::new(RefCell::new(TreeNode::new(v1)))),
                Some(Rc::new(RefCell::new(TreeNode::new(v2)))),
            ),
            _ => root,
        }
    }
}

use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Trie {
    end: bool,
    m: Rc<RefCell<HashMap<u8, Trie>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self {
            m: Rc::new(RefCell::new(HashMap::new())),
            end: false,
        }
    }

    fn insert(&self, word: String) {
        match word.len() {
            0 => (),
            l => {
                let end = l == 1;
                let c = word.bytes().nth(0).unwrap();
                self.m
                    .borrow_mut()
                    .entry(c)
                    .and_modify(|v| v.end = v.end || end)
                    .or_insert(Self { end, ..Self::new() });
                let a = Rc::clone(&self.m);
                let a = a.borrow();
                Self::insert(a.get(&c).unwrap(), word[1..].to_string())
            }
        }
    }

    fn search(&self, word: String) -> bool {
        match word.len() {
            0 => self.end,
            _ => {
                let c = word.bytes().nth(0).unwrap();
                match self.m.borrow().get(&c) {
                    Some(v) => Self::search(v, word[1..].to_string()),
                    None => false,
                }
            }
        }
    }

    fn starts_with(&self, prefix: String) -> bool {
        match prefix.len() {
            0 => true,
            _ => {
                let c = prefix.bytes().nth(0).unwrap();
                match self.m.borrow().get(&c) {
                    Some(v) => Self::starts_with(v, prefix[1..].to_string()),
                    None => false,
                }
            }
        }
    }
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
struct WordDictionary {
    end: bool,
    m: Rc<RefCell<HashMap<u8, WordDictionary>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        Self {
            m: Rc::new(RefCell::new(HashMap::new())),
            end: false,
        }
    }

    fn add_word(&self, word: String) {
        match word.len() {
            0 => (),
            l => {
                let end = l == 1;
                let c = word.bytes().nth(0).unwrap();
                self.m
                    .borrow_mut()
                    .entry(c)
                    .and_modify(|v| v.end = v.end || end)
                    .or_insert(Self { end, ..Self::new() });
                let a = Rc::clone(&self.m);
                let a = a.borrow();
                Self::add_word(a.get(&c).unwrap(), word[1..].to_string())
            }
        }
    }

    fn search(&self, word: String) -> bool {
        match word.len() {
            0 => self.end,
            _ => {
                let c = word.bytes().nth(0).unwrap();
                let target = word[1..].to_string();
                match c {
                    b'.' => self
                        .m
                        .borrow()
                        .iter()
                        .any(move |(_, v)| v.search(target.clone())),
                    _ => match self.m.borrow().get(&c) {
                        Some(v) => Self::search(v, target),
                        None => false,
                    },
                }
            }
        }
    }
}

struct Node {
    m: Rc<RefCell<HashMap<char, Node>>>,
}

impl Node {
    fn new() -> Self {
        Self {
            m: Rc::new(RefCell::new(HashMap::new())),
        }
    }
    fn new_with_c(c: char) -> Self {
        let mut hm = HashMap::new();
        hm.insert(c, Self::new());
        Self {
            m: Rc::new(RefCell::new(hm)),
        }
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let map = board
            .iter()
            .map(|x| x.iter().map(|x| Node::new_with_c(*x)).collect())
            .collect::<Vec<Vec<Node>>>();

        for i in 0..board.len() {
            for j in 0..board[i].len() {
            }
        }
        Vec::new()
    }
}

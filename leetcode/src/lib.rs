#![feature(array_zip)]
#![feature(test)]
#![allow(dead_code)]

struct Solution;
use std::collections::HashMap;
use std::collections::HashSet;
extern crate test;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm = HashMap::new();
        let res = nums.iter().enumerate().try_for_each(|(i, &v)| {
            if let Some(&i0) = hm.get(&(target - v)) {
                return Break((i0, i as i32));
            }
            hm.insert(v, i as i32);
            Continue(())
        });
        if let Break((a, b)) = res {
            return vec![a, b];
        }
        vec![]
    }
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }
        let (mut res, mut min) = (0, prices[0]);
        for &v in prices[1..].iter() {
            res = (v - min).max(res);
            min = v.min(min);
        }
        res
    }
}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hs = HashSet::new();
        if let Break(_) = nums.iter().try_for_each(|v| {
            if hs.contains(v) {
                return Break(true);
            }
            hs.insert(v);
            Continue(())
        }) {
            return true;
        }
        false
    }
}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        match len {
            0 => return vec![],
            1 => return vec![0],
            _ => (),
        }
        let (mut a, mut b, mut res) = (1, 1, vec![1; len]);
        (1..len).for_each(|i| {
            a *= nums[i - 1];
            res[i] *= a;
            b *= nums[len - i];
            res[len - i - 1] *= b;
        });
        res
    }
}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let (mut pre, mut res) = (nums[0], nums[0]);
        nums[1..].iter().for_each(|&v| {
            res = res.max(pre.max(0) + v);
            pre = pre.max(0) + v;
        });
        res
    }
}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let (mut res, mut min, mut max) = (nums[0], nums[0], nums[0]);
        nums[1..].iter().for_each(|&v| {
            if v < 0 {
                std::mem::swap(&mut min, &mut max);
            }
            max = (v * max).max(v);
            min = (v * min).min(v);
            res = res.max(max);
        });
        res
    }
}

use std::ops::ControlFlow::{Break, Continue};
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        match len {
            0 => return 0,
            1 => return nums[0],
            _ => (),
        }
        if let Break(res) = (1..len).try_for_each(|i| {
            if nums[i] < nums[i - 1] {
                return Break(nums[i]);
            }
            Continue(())
        }) {
            return res;
        }
        return nums[0];
    }
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        let len = nums.len();
        let mut offset = 0_usize;
        if let Break(x) = (1..len).try_for_each(|i| {
            if nums[i] < nums[i - 1] {
                return Break(i);
            }
            Continue(())
        }) {
            offset = x
        }
        nums.sort();
        if let Some((i, _)) = nums.iter().enumerate().find(|&(_, &v)| v == target) {
            return ((i + offset) % len) as i32;
        }
        -1
    }
}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        match len {
            0 | 1 | 2 => return vec![],
            _ => (),
        }
        let mut nums = nums;
        nums.sort();
        let mut hs: HashSet<[i32; 3]> = HashSet::new();
        nums[..len - 2].iter().enumerate().for_each(|(i, &v)| {
            let (mut left, mut right) = (i + 1, len - 1);
            while left < right {
                match (v + nums[left] + nums[right]).cmp(&0) {
                    std::cmp::Ordering::Less => left += 1,
                    std::cmp::Ordering::Greater => right -= 1,
                    _ => {
                        hs.insert([v, nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                    }
                }
            }
        });

        hs.iter().map(|&x| Vec::from(x)).collect()
    }
}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len();
        match len {
            0 | 1 => return 0,
            2 => return height[0].min(height[1]),
            _ => (),
        }

        let (mut left, mut right, mut res) = (0, len - 1, 0);
        while left < right {
            res = res.max(height[left].min(height[right]) * (right - left) as i32);
            match height[left].cmp(&height[right]) {
                std::cmp::Ordering::Less => left += 1,
                _ => right -= 1,
            }
        }
        res
    }
}

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let (mut a, mut b) = (a, b);
        while b != 0 {
            let t = a;
            a = a ^ b;
            b = (t & b) << 1;
        }
        a
    }
}

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let (mut res, mut n) = (0, n);
        while n != 0 {
            res += n & 1;
            n >>= 1;
        }
        res as i32
    }
}

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut dp = vec![];
        (0..n + 1).for_each(|i| match i {
            0 => dp.push(0),
            _ => dp.push(dp[(i & (i - 1)) as usize] + 1),
        });
        dp
    }
}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        nums.iter().enumerate().for_each(|(i, &v)| {
            res ^= (i as i32) ^ v;
        });
        res ^= nums.len() as i32;
        res
    }
}

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let (mut res, mut n, mut x) = (0, 31_u32, x);
        while x != 0 {
            res += (x & 1) << n;
            x >>= 1;
            n -= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn two_sum() {
        let input = [(vec![2, 7, 11, 15], 9), (vec![3, 2, 4], 6), (vec![3, 3], 6)];
        let want = [vec![0, 1], vec![1, 2], vec![0, 1]];

        input.zip(want).iter().for_each(|(a, b)| {
            let got = Solution::two_sum(a.0.to_owned(), a.1);
            assert_eq!(got, b.to_owned())
        })
    }

    #[test]
    fn max_profit() {
        let input = [vec![7, 1, 5, 3, 6, 4], vec![7, 6, 4, 3, 1]];
        let want = [5, 0];

        input.zip(want).iter().for_each(|(a, b)| {
            let got = Solution::max_profit(a.to_owned());
            assert_eq!(got, b.to_owned())
        })
    }

    #[test]
    fn contains_duplicate() {
        let input = [
            vec![1, 2, 3, 1],
            vec![1, 2, 3, 4],
            vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2],
        ];
        let want = [true, false, true];

        input.zip(want).iter().for_each(|(a, b)| {
            let got = Solution::contains_duplicate(a.to_owned());
            assert_eq!(got, b.to_owned())
        })
    }
    #[test]
    fn product_except_self() {
        let input = [vec![1, 2, 3, 4], vec![-1, 1, 0, -3, 3]];
        let want = [vec![24, 12, 8, 6], vec![0, 0, 9, 0, 0]];

        input.zip(want).iter().for_each(|(a, b)| {
            let got = Solution::product_except_self(a.to_owned());
            assert_eq!(got, b.to_owned())
        })
    }

    #[bench]
    fn bench_product_except_self(b: &mut test::Bencher) {
        b.iter(|| {
            assert_eq!(
                Solution::product_except_self(vec![1, 2, 3, 4]),
                vec![24, 12, 8, 6]
            )
        })
    }
    #[test]
    fn max_sub_array() {
        let input = [
            vec![-2, 1, -3, 4, -1, 2, 1, -5, 4],
            vec![1],
            vec![5, 4, -1, 7, 8],
        ];
        let want = [6, 1, 23];

        input.zip(want).iter().for_each(|(a, b)| {
            let got = Solution::max_sub_array(a.to_owned());
            assert_eq!(got, b.to_owned())
        })
    }

    #[test]
    fn max_product() {
        let input = [
            vec![2, 3, -2, 4],
            vec![-2, 0, -1],
            vec![-2, -3, 4],
            vec![-2, 3, -4],
        ];
        let want = [6, 0, 24, 24];

        input.zip(want).iter().for_each(|(a, b)| {
            let got = Solution::max_product(a.to_owned());
            assert_eq!(got, b.to_owned())
        })
    }

    #[test]
    fn find_min() {
        let input = [vec![4, 5, 6, 7, 0, 1, 2]];
        let want = [0];

        input.zip(want).iter().for_each(|(a, b)| {
            let got = Solution::find_min(a.to_owned());
            assert_eq!(got, b.to_owned())
        })
    }

    #[test]
    fn search() {
        let input = [
            (vec![4, 5, 6, 7, 0, 1, 2], 4),
            (vec![4, 5, 6, 7, 0, 1, 2], 3),
            (vec![1], 0),
        ];
        let want = [0, -1, -1];

        input.zip(want).iter().for_each(|(a, b)| {
            let got = Solution::search(a.0.to_owned(), a.1);
            assert_eq!(got, b.to_owned())
        })
    }

    #[test]
    fn three_sum() {
        let input = [(vec![-1, 0, 1, 2, -1, -4]), (vec![]), (vec![0])];
        let want = [vec![vec![-1, -1, 2], vec![-1, 0, 1]], vec![], vec![]];

        input.zip(want).iter().for_each(|(a, b)| {
            let got = Solution::three_sum(a.to_owned());
            assert_eq!(got, b.to_owned())
        })
    }
}

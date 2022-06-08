struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        match b {
            0 => a,
            _ => Solution::get_sum(a ^ b, (a & b) << 1),
        }
    }
}

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            _ => (n & 1) as i32 + Self::hammingWeight(n >> 1),
        }
    }
}

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = vec![0];
        (1..=n).for_each(|x| res.push(res[(x & (x - 1)) as usize] + 1));
        res
    }
}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        nums.iter()
            .enumerate()
            .for_each(|(i, &v)| res ^= i as i32 ^ v);

        res ^ (nums.len() as i32)
    }
}

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let (mut n, mut x, mut res) = (31_u32, x, 0_u32);
        while x != 0 {
            res += (x & 1) << n;
            x >>= 1;
            n -= 1;
        }
        res
    }
}

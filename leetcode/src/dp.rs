struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (1, 2);
        (1..n).for_each(|_| {
            let temp = a;
            a = b;
            b = temp + b;
        });
        a
    }
}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![-1; amount as usize + 1];
        dp[0] = 0;
        (1..=amount).for_each(|i| {
            coins.iter().for_each(|&c| {
                if i >= c {
                    let pre = dp[(i - c) as usize];
                    if pre != -1 {
                        let i = i as usize;
                        match dp[i] {
                            -1 => dp[i] = pre + 1,
                            _ => dp[i] = dp[i].min(pre + 1),
                        }
                    }
                }
            });
        });
        dp[amount as usize]
    }
}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        if l == 0 {
            return 0;
        }
        let mut dp = vec![1; l];
        let mut res = 1;
        (1..l).for_each(|i| {
            dp[i] = (&dp[..i])
                .iter()
                .enumerate()
                .filter(|(x, _)| nums[*x] < nums[i])
                .map(|(_, &v)| v)
                .max()
                .unwrap_or_default()
                + 1;
            res = res.max(dp[i])
        });

        res as i32
    }
}
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (l1, l2) = (text1.len() + 1, text2.len() + 1);
        let mut dp = vec![vec![0; l2]; l1];
        (1..l1).for_each(|i| {
            (1..l2).for_each(|j| {
                dp[i][j] = match text1.bytes().nth(i - 1) == text2.bytes().nth(j - 1) {
                    true => dp[i - 1][j - 1] + 1,
                    false => (dp[i - 1][j]).max(dp[i][j - 1]),
                }
            })
        });
        dp[l1 - 1][l2 - 1]
    }
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;
        (1..dp.len()).for_each(|i| {
            word_dict.iter().try_for_each(|word| {
                if word.len() <= i {
                    let index = i - word.len();
                    if dp[index] && &s[index..i] == word {
                        dp[i] = true;
                        return std::ops::ControlFlow::Break(());
                    }
                }
                std::ops::ControlFlow::Continue(())
            });
        });
        dp.last().unwrap_or(&false).clone()
    }
}

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        (1..dp.len()).for_each(|i| {
            nums.iter().for_each(|&num| {
                let num = num as usize;
                if i >= num {
                    dp[i] += dp[i - num]
                }
            });
        });

        dp.last().unwrap_or(&0).clone()
    }
}

impl Solution {
    pub fn rob2(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        let mut dp = vec![0; length];
        (0..length).for_each(|i| {
            dp[i] = match i {
                0 => nums[0],
                1 => nums[0].max(nums[1]),
                x => dp[x - 1].max(nums[x] + dp[x - 2]),
            }
        });
        dp[length - 1]
    }
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            len => Self::rob2((&nums[1..]).to_vec()).max(Self::rob2((&nums[..(len - 1)]).to_vec())),
        }
    }
}
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut dp = vec![0; s.len()];
        s.bytes().enumerate().try_for_each(|(i, c)| {
            dp[i] = match i {
                0 => (c != b'0') as i32,
                1 => match (s.bytes().nth(i - 1).unwrap(), c) {
                    (b'1', b'1'..=b'9') | (b'2', b'1'..=b'6') => 2,
                    (b'0', b'1'..=b'9') | (b'1', _) | (b'2', _) | (b'3'..=b'9', b'1'..=b'9') => 1,
                    _ => 0,
                },
                _ => match (s.bytes().nth(i - 1).unwrap(), c) {
                    (b'1', b'1'..=b'9') | (b'2', b'1'..=b'6') => dp[i - 1] + dp[i - 2],
                    (b'0', b'1'..=b'9') | (b'2', b'7'..=b'9') | (b'3'..=b'9', b'1'..=b'9') => {
                        dp[i - 1]
                    }
                    (b'1', _) | (b'2', _) => dp[i - 2],
                    _ => 0,
                },
            };
            match dp[i] {
                0 => std::ops::ControlFlow::Break(()),
                _ => std::ops::ControlFlow::Continue(()),
            }
        });

        dp.last().unwrap_or(&0).clone()
    }
}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0; n + 1]; m + 1];
        (1..=m).for_each(|i| {
            (1..=n).for_each(|j| {
                dp[i][j] = match (i, j) {
                    (1, 1) => 1,
                    _ => dp[i - 1][j] + dp[i][j - 1],
                }
            })
        });
        dp.last().unwrap().last().unwrap().clone()
    }
}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let (mut left, mut right, len) = (0, 0, nums.len());
        if len == 0 {
            return false;
        }
        while left <= right && right < len {
            right = right.max(left + nums[left] as usize);
            left += 1
        }
        right >= len - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_list() {
        let want = 4;
        let got = Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]);

        assert_eq!(got, want);
    }
}

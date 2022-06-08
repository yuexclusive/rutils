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
        let dp = vec![1; l];
        let res = 1;
        (1..l).for_each(|i| {
            let max: Vec<&usize> = (&dp[..i]).iter().filter(|&&x| nums[x] < nums[i]).collect();
            println!("{:?}", i);
            println!("{:?}", max);
            // dp[i] =
            //     .unwrap_or(&0)
            //     + 1;
            // println!("dp[{}]: {}", i, dp[i]);
            // res = res.max(dp[i])
        });

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_lis() {
        let want = 4;
        let got = Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]);

        assert_eq!(got, want);
    }
}

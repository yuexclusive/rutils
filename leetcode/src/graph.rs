struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut m1 = vec![0; num_courses as usize];
        let mut m2: HashMap<i32, Vec<i32>> = HashMap::new();

        prerequisites.iter().for_each(|v| {
            m1[v[0] as usize] += 1;
            m2.entry((&v[1]).clone())
                .and_modify(|x| x.push(v[0]))
                .or_insert(vec![v[0]]);
        });
        let mut res = 0;
        loop {
            if let Some((i, _)) = m1.iter().enumerate().filter(|(_, v)| **v == 0).next() {
                m1[i] = -1;
                res += 1;
                if let Some((_, v)) = m2.remove_entry(&(i as i32)) {
                    v.iter().for_each(|j| {
                        m1[*j as usize] -= 1;
                    });
                }
                continue;
            }
            break;
        }
        res == num_courses
    }
}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (l1, l2) = (heights.len(), heights[0].len());
        let mut map = vec![vec![0; l2]; l1];
        (0..l1)
            .map(|i| (i, 0))
            .chain((0..l2).map(|j| (0, j)))
            .for_each(|(i, j)| {
                Self::walk(&heights, &mut map, Some(i), Some(j), i, j, 1);
            });

        (0..l1)
            .map(|i| (i, l2 - 1))
            .chain((0..l2).map(|j| (l1 - 1, j)))
            .for_each(|(i, j)| {
                Self::walk(&heights, &mut map, Some(i), Some(j), i, j, 2);
            });

        let mut res = Vec::new();

        map.iter().enumerate().for_each(|(i, x)| {
            x.iter().enumerate().for_each(|(j, t)| {
                if *t == 3 {
                    res.push(vec![i as i32, j as i32]);
                }
            });
        });

        res
    }

    fn walk(
        heights: &Vec<Vec<i32>>,
        map: &mut Vec<Vec<i32>>,
        i: Option<usize>,
        j: Option<usize>,
        i0: usize,
        j0: usize,
        val: i32,
    ) {
        if i.is_none() || j.is_none() {
            return;
        }
        let (i, j) = (i.unwrap(), j.unwrap());
        if i >= heights.len()
            || j >= heights[i].len()
            || heights[i][j] < heights[i0][j0]
            || map[i][j] >= val
        {
            return;
        }
        map[i][j] += val;

        Self::walk(heights, map, Some(i + 1), Some(j), i, j, val);
        Self::walk(heights, map, i.checked_sub(1), Some(j), i, j, val);
        Self::walk(heights, map, Some(i), Some(j + 1), i, j, val);
        Self::walk(heights, map, Some(i), j.checked_sub(1), i, j, val);
    }
}

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut hm = HashSet::new();
        for &item in &nums {
            hm.insert(item);
        }
        let mut res = 0;
        for &item in &nums {
            let mut val = item - 1;
            let mut i = 1;
            while hm.remove(&val) {
                i += 1;
                val -= 1;
            }
            val = item + 1;
            while hm.remove(&val) {
                i += 1;
                val += 1;
            }
            res = res.max(i);
        }
        res
    }
}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut last = new_interval;
        let mut res = Vec::new();
        intervals.iter().for_each(|x| {
            if x[1] < last[0] || x[0] > last[1] {
                res.push(x.clone())
            } else {
                last[0] = last[0].min(x[0]);
                last[1] = last[1].max(x[1]);
            }
        });
        res.push(last);
        res.sort_by(|a, b| a[0].cmp(&b[0]));
        res
    }
}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut res = vec![intervals.get(0).unwrap().clone()];
        intervals.iter().skip(1).for_each(|x| {
            let last = res.last_mut().unwrap();
            if x[0] > last[1] {
                res.push(x.clone());
            } else {
                last[0] = x[0].min(last[0]);
                last[1] = x[1].max(last[1]);
            }
        });
        res
    }
}

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0]).reverse());
        let mut last = intervals.iter().next().unwrap().clone();
        let mut res = 0;
        intervals.iter().skip(1).for_each(|v| {
            if v[1] > last[0] {
                res += 1;
            } else {
                last = v.clone()
            }
        });
        res
    }
}

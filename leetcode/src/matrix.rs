struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut s1 = HashSet::new();
        let mut s2 = HashSet::new();
        matrix.iter().enumerate().for_each(|(i, v)| {
            v.iter().enumerate().for_each(|(j, &v)| {
                if v == 0 {
                    s1.insert(i);
                    s2.insert(j);
                }
            });
        });

        matrix.iter_mut().enumerate().for_each(|(i, v)| {
            v.iter_mut().enumerate().for_each(|(j, v)| {
                if s1.get(&i).is_some() || s2.get(&j).is_some() {
                    *v = 0
                }
            });
        });
    }
}

enum Direction {
    Right,
    Left,
    Down,
    Up,
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();

        let (mut i, mut j) = (0, 0);
        let (mut left, mut right, mut top, mut bottom) = (
            0 as i32,
            (matrix[0].len() - 1) as i32,
            0 as i32,
            (matrix.len() - 1) as i32,
        );
        let mut direction = Direction::Right;
        let mut insert = true;
        while left <= right && top <= bottom {
            if insert {
                res.push(matrix[i][j]);
            }
            insert = false;
            match direction {
                Direction::Right => {
                    if j as i32 == right {
                        direction = Direction::Down;
                        top += 1;
                    } else {
                        j += 1;
                        insert = true;
                    }
                }
                Direction::Down => {
                    if i as i32 == bottom {
                        direction = Direction::Left;
                        right -= 1;
                    } else {
                        i += 1;
                        insert = true;
                    }
                }
                Direction::Left => {
                    if j as i32 == left {
                        direction = Direction::Up;
                        bottom -= 1;
                    } else {
                        j -= 1;
                        insert = true;
                    }
                }
                Direction::Up => {
                    if i as i32 == top {
                        direction = Direction::Right;
                        left += 1;
                    } else {
                        i -= 1;
                        insert = true;
                    }
                }
            }
        }

        res
    }
}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let mut new_matrix = matrix.clone();
        for (i, v) in matrix.iter().enumerate() {
            for (j, v) in v.iter().enumerate() {
                new_matrix[j][n - i - 1] = *v
            }
        }
        *matrix = new_matrix
    }
}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut map = vec![vec![false; board[0].len()]; board.len()];
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if Self::walk(&board, &mut map, &word, 0, Some(i), Some(j)) {
                    return true;
                }
            }
        }

        false
    }

    pub fn walk(
        board: &Vec<Vec<char>>,
        map: &mut Vec<Vec<bool>>,
        word: &str,
        index: usize,
        i: Option<usize>,
        j: Option<usize>,
    ) -> bool {
        if index == word.len() {
            return true;
        }

        if i.is_none() || j.is_none() {
            return false;
        }

        let (i, j) = (i.unwrap(), j.unwrap());

        if i >= board.len()
            || j >= board[i].len()
            || map[i][j]
            || word.chars().nth(index).unwrap() != board[i][j]
        {
            return false;
        }
        let index = index + 1;
        map[i][j] = true;
        let res = Self::walk(board, map, word, index, i.checked_add(1), Some(j))
            || Self::walk(board, map, word, index, i.checked_sub(1), Some(j))
            || Self::walk(board, map, word, index, Some(i), j.checked_add(1))
            || Self::walk(board, map, word, index, Some(i), j.checked_sub(1));
        map[i][j] = false;
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exist() {
        let res = Solution::exist(
            vec![
                vec!['C', 'A', 'A'],
                vec!['A', 'A', 'A'],
                vec!['B', 'C', 'D'],
            ],
            "AAB".to_string(),
        );

        assert_eq!(res, true);
    }
}

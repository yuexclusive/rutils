#![allow(dead_code)]

pub fn bubble_sort(slice: &mut Vec<i32>) {
    let (mut length, mut forward) = (slice.len() as isize, true);
    while forward {
        forward = false;
        for i in 0..length - 1 {
            let i = i as usize;
            if slice[i].gt(&slice[i + 1]) {
                (slice[i], slice[i + 1]) = (slice[i + 1], slice[i]);
                forward = true
            }
        }
        length -= 1;
    }
}

pub fn select_sort(slice: &mut Vec<i32>) {
    let length = slice.len() as isize;
    for i in 0..length - 1 {
        let i = i as usize;
        for j in i + 1..slice.len() {
            if slice[i].gt(&slice[j]) {
                (slice[i], slice[j]) = (slice[j], slice[i])
            }
        }
    }
}

pub fn insert_sort(slice: &mut Vec<i32>) {
    for i in 1..slice.len() {
        for j in (1..i + 1).rev() {
            if slice[j].lt(&slice[j - 1]) {
                (slice[j], slice[j - 1]) = (slice[j - 1], slice[j])
            } else {
                break;
            }
        }
    }
}

pub fn hill_sort(slice: &mut Vec<i32>) {
    let mut i = slice.len() / 2;
    while i >= 1 {
        for j in 0..i {
            for k in ((j + i)..slice.len()).step_by(i) {
                for l in (j + i..k + 1).rev().step_by(i) {
                    if slice[l - i].gt(&slice[l]) {
                        (slice[l - i], slice[l]) = (slice[l], slice[l - i])
                    } else {
                        break;
                    }
                }
            }
        }
        i /= 2;
    }
}

pub fn quick_sort(slice: &mut Vec<i32>) {
    quick_sort_internal(slice, 0, slice.len().max(1) - 1);
}

fn quick_sort_internal(slice: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        return;
    }
    let (mut start, mut end, mut flag) = (left, right, left);
    while start < end {
        while start < end {
            if slice[flag].gt(&slice[end]) {
                (slice[flag], slice[end]) = (slice[end], slice[flag]);
                flag = end;
                break;
            }
            end -= 1;
        }
        while start < end {
            if slice[start].gt(&slice[flag]) {
                (slice[flag], slice[start]) = (slice[start], slice[flag]);
                flag = start;
                break;
            }
            start += 1;
        }
    }
    quick_sort_internal(slice, left, flag - 1);
    quick_sort_internal(slice, flag + 1, right);
}

pub fn merge_sort(slice: &mut Vec<i32>) {
    *slice = merge_sort_internal(slice.to_owned());
}

fn merge_sort_internal(slice: Vec<i32>) -> Vec<i32> {
    let length = slice.len();
    if length <= 1 {
        return slice;
    }
    let mid = length / 2;
    let left = Vec::from(&slice[..mid]);
    let right = Vec::from(&slice[mid..]);
    merge(merge_sort_internal(left), merge_sort_internal(right))
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let (mut res, mut p1, mut p2) = (Vec::with_capacity(left.len() + right.len()), 0, 0);
    while p1 < left.len() && p2 < right.len() {
        if left[p1].lt(&right[p2]) {
            res.push(left[p1]);
            p1 += 1;
            continue;
        }
        res.push(right[p2]);
        p2 += 1
    }
    res.append(&mut left[p1..].to_vec());
    res.append(&mut right[p2..].to_vec());
    res
}

pub fn heap_sort(slice: &mut Vec<i32>) {
    let mut end = slice.len().max(1) - 1;
    for i in (0..(end / 2 + 1)).rev() {
        rise(slice, i, end);
    }
    while end > 0 {
        (slice[0], slice[end]) = (slice[end], slice[0]);
        end -= 1;
        rise(slice, 0, end)
    }
}

pub fn rise(slice: &mut Vec<i32>, k: usize, end: usize) {
    let mut k = k;
    loop {
        let mut i = 2 * k + 1;
        if i > end {
            break;
        }
        if i < end && slice[i + 1].gt(&slice[i]) {
            i += 1;
        }
        if slice[k].ge(&slice[i]) {
            break;
        }
        (slice[k], slice[i]) = (slice[i], slice[k]);
        k = i;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    fn get_map() -> HashMap<Vec<i32>, Vec<i32>> {
        let mut hm = HashMap::new();

        hm.insert(vec![3, 8, 6, 4, 7, 1, 2, 5], vec![1, 2, 3, 4, 5, 6, 7, 8]);
        hm.insert(vec![1], vec![1]);
        hm.insert(vec![], vec![]);

        hm
    }
    fn test_sort<T>(f: T)
    where
        T: Fn(&mut Vec<i32>),
    {
        let hm = get_map();
        for (slice, want) in hm {
            let mut slice = slice;
            f(&mut slice);
            assert_eq!(slice, want);
        }
    }
    #[test]
    fn test_bubble_sort() {
        test_sort(bubble_sort)
    }

    #[test]
    fn test_select_sort() {
        test_sort(select_sort)
    }

    #[test]
    fn test_insert_sort() {
        test_sort(insert_sort)
    }

    #[test]
    fn test_hill_sort() {
        test_sort(hill_sort)
    }

    #[test]
    fn test_quick_sort() {
        test_sort(quick_sort)
    }

    #[test]
    fn test_merge_sort() {
        test_sort(merge_sort)
    }

    #[test]
    fn test_heap_sort() {
        test_sort(heap_sort)
    }

}

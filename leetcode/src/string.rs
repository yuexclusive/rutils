struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut left, mut right, mut res, bs) = (0, 0, 1, s.bytes().collect::<Vec<u8>>());
        match bs.len() {
            0 => 0,
            1 => 1,
            _ => {
                loop {
                    res = res.max(right - left + 1);
                    right += 1;
                    if right >= bs.len() {
                        break;
                    }
                    for i in (left..right).rev() {
                        if bs[i] == bs[right] {
                            left = i + 1;
                            break;
                        }
                    }
                }
                res as i32
            }
        }
    }
}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bs = s.bytes().collect::<Vec<u8>>();
        match bs.len() {
            0 => 0,
            1 => 1,
            _ => {
                if bs.len() - 1 <= k as usize {
                    return bs.len() as i32;
                }
                let (mut left, mut right, mut res) = (0_usize, 0_usize, 1);
                let mut hm = HashMap::new();
                let get_max = |hm: &HashMap<u8, i32>| hm.iter().map(|x| x.1).max().unwrap().clone();
                hm.insert(bs[0], 1);
                loop {
                    let max = get_max(&hm);
                    if (right - left + 1) as i32 - max <= k {
                        res = res.max(right - left + 1);
                        right += 1;
                        if right >= bs.len() {
                            break;
                        }
                        hm.entry(bs[right]).and_modify(|v| *v += 1).or_insert(1);
                        continue;
                    }
                    hm.entry(bs[left]).and_modify(|v| *v -= 1);
                    left += 1;
                }
                res as i32
            }
        }
    }
}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        match (s.len(), t.len()) {
            (0, _) | (_, 0) => "".to_string(),
            _ => {
                let mut res = "";
                let check = |hm: &HashMap<u8, i32>| -> bool { !hm.iter().any(|x| *x.1 > 0) };
                if s.len() < t.len() {
                    return "".to_string();
                }
                let (mut left, mut right) = (0, 0);
                let mut hm = HashMap::new();
                for c in t.bytes() {
                    hm.entry(c).and_modify(|v| *v += 1).or_insert(1);
                }
                let bs = s.bytes().collect::<Vec<u8>>();
                hm.entry(bs[0]).and_modify(|v| *v -= 1);
                loop {
                    if check(&hm) {
                        if res == "" || res.len() >= right - left + 1 {
                            res = &s[left..right + 1];
                        }
                        hm.entry(bs[left]).and_modify(|v| *v += 1);
                        left += 1;
                        continue;
                    }
                    right += 1;
                    if right >= bs.len() {
                        break;
                    }
                    hm.entry(bs[right]).and_modify(|v| *v -= 1);
                }
                res.to_string()
            }
        }
    }
}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut hm = HashMap::new();
        s.bytes().for_each(|x| {
            hm.entry(x).and_modify(|v| *v += 1).or_insert(1);
        });
        t.bytes().for_each(|x| {
            hm.entry(x).and_modify(|v| *v -= 1).or_insert(-1);
        });
        hm.iter().all(|x| *x.1 == 0)
    }
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = Vec::with_capacity(26);
        let mut start = 3;
        for _ in 0..26 {
            'l: loop {
                for i in 2..(start / 2 + 1) {
                    if start % i == 0 {
                        start += 1;
                        continue 'l;
                    }
                }
                break;
            }
            map.push(start);
            start += 1;
        }
        let mut hm = HashMap::new();

        for str in &strs {
            let mut key = 1;
            str.bytes().for_each(|x| {
                key *= map[(x - b'a') as usize];
            });
            hm.entry(key)
                .and_modify(|v: &mut Vec<String>| v.push(str.clone()))
                .or_insert(vec![str.clone()]);
        }

        hm.into_iter().map(|(_, v)| v).collect()
    }
}

use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut vq = VecDeque::new();
        let mut hm = HashMap::new();
        hm.insert(b'}', b'{');
        hm.insert(b')', b'(');
        hm.insert(b']', b'[');
        for x in s.bytes() {
            match x {
                b'{' | b'[' | b'(' => vq.push_back(x),
                _ => {
                    if let (Some(&v), Some(v2)) = (hm.get(&x), vq.pop_back()) {
                        if v == v2 {
                            continue;
                        }
                    }
                    return false;
                }
            }
        }
        vq.len() == 0
    }
}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bs = s
            .bytes()
            .filter(|x| x.is_ascii_alphanumeric())
            .map(|x| x.to_ascii_lowercase())
            .collect::<Vec<u8>>();
        for (i, &v) in (&bs[..bs.len() / 2]).iter().enumerate() {
            if v != bs[bs.len() - i - 1] {
                return false;
            }
        }
        true
    }
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut res = "";
        let bs = s.bytes().collect::<Vec<u8>>();
        for i in 0..bs.len() {
            for mut right in i..=i + 1 {
                let mut left = i;
                while right < bs.len() {
                    if bs[left] != bs[right] {
                        break;
                    } else {
                        if res == "" || res.len() < right - left + 1 {
                            res = &s[left..right + 1]
                        }
                    }
                    if left == 0 {
                        break;
                    }
                    left -= 1;
                    right += 1;
                }
            }
        }

        res.to_string()
    }
}

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut res = 0;
        let bs = s.bytes().collect::<Vec<u8>>();
        for i in 0..bs.len() {
            for mut right in i..=i + 1 {
                let mut left = i;
                while right < bs.len() {
                    if bs[left] != bs[right] {
                        break;
                    } else {
                        res += 1
                    }
                    if left == 0 {
                        break;
                    }
                    left -= 1;
                    right += 1;
                }
            }
        }

        res
    }
}

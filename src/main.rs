#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

// https://leetcode.com/problems/majority-element/
fn majority_element(arr: Vec<i32>) -> i32 {
    let threshold = arr.len() / 2;
    let map = arr.iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(*x).or_insert(0) += 1;
        acc
    });
    let maybe_majority_entry = map.iter().find(|&(_, freq)| *freq > threshold);
    match maybe_majority_entry {
        Some(majority_entry) => match majority_entry {
            (majority, _) => *majority
        },
        None => panic!("No majority element found")
    }
}

// https://leetcode.com/problems/unique-number-of-occurrences/
fn unique_occurrences(nums: Vec<i32>) -> bool {
    let map = nums.iter().fold(HashMap::new(), |mut acc, n| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    });
    let mut value_set = HashSet::new();
    println!("{:?}", map);
    map.iter().all(|(_, v)| value_set.insert(*v))
}

// https://leetcode.com/problems/valid-anagram/
fn is_anagram(s: &str, t: &str) -> bool {
    let exists = |entry: (&char, &i32), map: &HashMap<char, i32>| -> bool {
        match map.get(entry.0) {
            Some(freq) => *freq == *entry.1,
            None => false
        }
    };
    if s.len() == t.len() {
        let s_map = char_map(s);
        let t_map = char_map(t);
        return s_map.iter().all(|(c, freq)| exists((c, freq), &t_map));
    } else {
        false
    }
}

fn char_map(s: &str) -> HashMap<char, i32> {
    s.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}

// https://leetcode.com/problems/find-lucky-integer-in-an-array/
fn find_lucky(arr: Vec<i32>) -> i32 {
    let map = arr.iter().fold(HashMap::new(), |mut acc, n| {
        *acc.entry(*n).or_insert(0) += 1;
        acc
    });
    let lucky_nums: Vec<i32> = map.iter()
        .filter(|&(k, v)| *k == *v)
        .map(|(k, _)| *k)
        .collect();
    if lucky_nums.len() > 0 {
        match lucky_nums.iter().max() {
            Some(max) => *max,
            None => panic!("should not have gotten here")
        }
    } else {
        -1
    }
}

// https://leetcode.com/problems/two-sum/
fn two_sum(arr: Vec<i32>, target: i32) -> (i32, i32) {
    let mut diff_index = HashMap::new();
    for (i, n) in arr.iter().enumerate() {
        diff_index.insert(target - *n, i);
    }
    for (i, n) in arr.iter().enumerate() {
        if diff_index.contains_key(&n) {
            return match diff_index.get(&n) {
                Some(j) => (i as i32, *j as i32),
                None => panic!("should not have gotten here")
            }
        }
    }
    panic!("{:?} failed to contain elements that sum up to: {}", arr, target)
}

// https://leetcode.com/problems/find-all-duplicates-in-an-array/
fn find_duplicates(arr: Vec<i32>) -> Vec<i32> {
    let freq = arr.iter().fold(HashMap::new(), |mut acc, n| {
        *acc.entry(*n).or_insert(0) += 1;
        acc
    });
    freq.iter().filter(|&(_, v)| *v != 1)
        .map(|(k, _)| *k)
        .collect()
}

// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/
// TODO: revisit this later, definitely not the nicest way to do it
fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    nums.iter().enumerate()
        .map(|(i, n)| {
            nums.iter().enumerate().filter(|&(j, x)| i != j && *x < *n)
                .map(|(_, _)| 1)
                .collect::<Vec<i32>>()
                .iter()
                .sum()
        })
        .collect()
}

// https://leetcode.com/problems/majority-element-ii/
fn other_majority_element(nums: Vec<i32>) -> Vec<i32> {
    let threshold = (nums.len() / 3) as i32;
    let freqs = nums.iter().fold(HashMap::new(), |mut acc, n| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    });
    freqs.iter()
        .filter(|&(_, v)| *v > threshold)
        .map(|(k, _)| **k)
        .collect()
}

fn single_number(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        nums[0]
    } else {
        let freqs = nums.iter().fold(HashMap::new(), |mut acc, n| {
            *acc.entry(n).or_insert(0) += 1;
            acc
        });
        match freqs.iter().find(|&(_, v)| *v == 1) {
            Some(entry) => match entry {
                (k, _) => **k
            },
            None => panic!("Single element does not exist in {:?}", nums)
        }
    }
}

fn count_elements(arr: Vec<i32>) -> i32 {
    arr.iter()
        .filter(|x| arr.contains(&(*x + 1)))
        .map(|_| 1)
        .sum()
}

// https://leetcode.com/problems/perfect-number/
fn check_perfect_number(num: i32) -> bool {
    if num != 0 {
        num == (1 .. num - 1).collect::<Vec<i32>>()
            .iter()
            .filter(|n| num % **n == 0)
            .sum()
    } else {
        false
    }
}

fn main() {
    println!("Test...test");
}

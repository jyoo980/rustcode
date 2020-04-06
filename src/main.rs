#![allow(dead_code)]
use std::collections::HashMap;

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

fn main() {
    println!("Hello, world!");
}

// use std::collections::HashMap;
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::{fmt::format, u16::MAX};
fn main() {
    println!("Hello, world!");
}
//49. Group Anagrams first medium problem
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    for string in strs {
        if result.is_empty() {
            let mut temp_vec: Vec<String> = Vec::new();
            temp_vec.push(string);
            result.push(temp_vec);
        } else {
            let mut group_index: u16 = 0;
            let mut string_index: u8 = 0;
            for group in &result {
                for string in group {}
            }
        }
    }
    return vec![vec!["s".to_string()]];
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let test: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let result: Vec<Vec<String>> = group_anagrams(test);
        assert_eq!(
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]],
            result
        );
    }
}

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
            result.push(vec![string]);
        } else {
            let mut group_index: i16 = -1;
            // fix the cloning cost
            let mut found_string = false;
            for group in &mut result {
                group_index += 1;
                // maybe i can find a way to cache this (in another array to win time for redendences)
                let mut characters: Vec<char> = group[0].chars().collect();
                characters.sort_unstable();
                let mut string_sorted: Vec<char> = string.clone().chars().collect();
                string_sorted.sort_unstable();
                let mut string_index: i8 = -1;

                if characters == string_sorted {
                    group.push(string.clone());
                    found_string = true;
                    break;
                }
            }
            if !found_string {
                result.push(vec![string]);
            }
        }
    }

    return result;
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

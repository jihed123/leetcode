// use std::collections::HashMap;
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::fmt::format;
fn main() {
    println!("Hello, world!");
}
//49. Group Anagrams first medium problem
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {}
#[cfg(test)]
mod tests {

    use std::fmt::Binary;

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

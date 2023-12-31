// use crate::hash_map::HashMap;
// use std::collections::hash_map;
// use std::collections::HashSet;
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]

// mod data;
// fn contains_duplicate(mut nums: Vec<i32>) -> bool {
//     nums.sort_unstable();
//     for i in 0..nums.len() - 1 {
//         if nums[i] == nums[i + 1] {
//             return true;
//         }
//     }
//     false
// }
// pub fn is_anagram(s: String, t: String) -> bool {
//     let mut hashset = HashSet::<char>::new();
//     hashset.extend(s.chars());
//     let mut hashset2 = HashSet::<char>::new();
//     hashset2.extend(t.chars());

//     // let result: Vec<char> = hashset.difference(&hashset2).cloned().collect();
//     let result: Vec<char> = hashset.symmetric_difference(&hashset2).cloned().collect();

//     return result.is_empty();
// }
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut s_sorted: Vec<char> = s.chars().collect::<Vec<char>>();
    let mut t_sorted: Vec<char> = t.chars().collect::<Vec<char>>();

    s_sorted.sort_unstable();
    t_sorted.sort_unstable();

    for i in 0..s_sorted.len() {
        if s_sorted[i] != t_sorted[i] {
            return false;
        }
    }

    return true;
    // 2.60MB Beats 9.07%of users with Rust
    // 0ms
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s: String = "anagram".to_string();
        let t: String = "nagaram".to_string();
        let result: bool = is_anagram(s, t);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let s: String = "rat".to_string();
        let t: String = "car".to_string();
        let result: bool = is_anagram(s, t);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let s: String = "a".to_string();
        let t: String = "ab".to_string();
        let result: bool = is_anagram(s, t);
        assert_eq!(result, false);
    }
    #[test]
    fn test_4() {
        let s: String = "a".to_string();
        let t: String = "aa".to_string();
        let result: bool = is_anagram(s, t);
        assert_eq!(result, false);
    }
}

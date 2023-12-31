// use crate::hash_map::HashMap;
// use std::collections::hash_map;
// use std::collections::HashSet;
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]

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
}

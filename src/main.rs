// use std::collections::HashMap;
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use std::collections::HashMap;
// 347. Top K Frequent Elements
fn main() {
    println!("Hello, world!");
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // after that i saw heap would be the solution i will try it out

    return Vec::new();
}

// ai solution
// pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
//     if nums.len() == k as usize {
//         return nums;
//     }

//     use std::collections::HashMap;
//     let mut hashmap: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
//     for i in 0..nums.len() {
//         *hashmap.entry(nums[i]).or_insert(0) += 1;
//     }

//     let mut pairs: Vec<_> = hashmap.into_iter().collect();
//     pairs.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

//     pairs
//         .into_iter()
//         .map(|(num, _)| num)
//         .take(k as usize)
//         .collect()

// let mut newvec: Vec<i32> = Vec::with_capacity(k as usize);
// let mut newvec: Vec<i32> = hashmap.values().cloned().collect();
// newvec.sort();
// newvec.truncate(k as usize);

// let mut result: Vec<i32> = Vec::with_capacity(k as usize);
// for value_to_find in newvec {
//     for (key, value) in &hashmap {
//         if value == &value_to_find {
//             result.push(*key);
//         }
//     }
// }

// order by values then get key by value then add to vec and send it

// dbg!(hashmap.keys());
// dbg!(hashmap.values());

// let vec1: Vec<_> = hashmap.values().collect();
// dbg!("val", vec1);

// make the hashmap have pointer value to it usize then make a few array with it.

// dbg!("{}", hashmap.keys(), hashmap.values());
// i have to give the two+ highest.
// }

#[cfg(test)]
mod tests {
    // use std::{borrow::Borrow, collections::btree_map::Entry};

    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let res = top_k_frequent(nums, k);
        assert_eq!(res, vec![1, 2]);
    }
}

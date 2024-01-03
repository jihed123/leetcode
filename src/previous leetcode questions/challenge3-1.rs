// use std::collections::HashMap;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index: usize = 0;
    let mut localindex: usize = 1;
    let len: usize = nums.len();
    loop {
        if index == len {
            return vec![0, 0];
        }
        while localindex < len {
            if index == localindex {
                localindex += 1;
                continue;
            }
            if nums[index] + nums[localindex] == target {
                return vec![index as i32, localindex as i32];
            }
            localindex += 1;
        }

        index += 1;
        localindex = 0;
    }
    // 60ms 2.19MB
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // use std::{borrow::Borrow, collections::btree_map::Entry};

    use super::*;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = [2, 7, 11, 15].to_vec();
        let target: i32 = 9;
        // get the index of two number that add up to it
        let result: Vec<i32> = two_sum(nums.clone(), target);

        let index1: usize = result[0] as usize;
        let index2: usize = result[1] as usize;
        let realvalue = nums[index1] + nums[index2];
        assert_eq!(realvalue, target);
    }
}

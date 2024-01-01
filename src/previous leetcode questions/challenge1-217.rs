#[allow(dead_code)]
mod data;
fn contains_duplicate(mut nums: Vec<i32>) -> bool {
    nums.sort_unstable();
    for i in 0..nums.len() - 1 {
        if nums[i] == nums[i + 1] {
            return true;
        }
    }
    false
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1, 2, 3, 1];
        let result: bool = contains_duplicate(nums);
        assert_eq!(result, true);
    }
    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        let result: bool = contains_duplicate(nums);
        assert_eq!(result, false);
    }
    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let result: bool = contains_duplicate(nums);
        assert_eq!(result, true);
    }
    #[test]
    fn test_4() {
        let nums = data::large_vec();
        let result: bool = contains_duplicate(nums);
        assert_eq!(result, false);
        // 56.34s
        // 50.28s with importing data from data.rs with a function
    }
}

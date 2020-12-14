#![allow(dead_code)]
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate1(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return false;
        }

        let mut tmp: Vec<i32> = Vec::new();
        for x in &nums {
            for y in &tmp {
                if x == y {
                    return true;
                }
            }
            tmp.push(*x);
        }
        false
    }

    pub fn contains_duplicate2(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return false;
        }

        let mut hm = HashMap::new();
        for x in &nums {
            let count = hm.entry(x).or_insert(0);
            *count += 1;
            if *count > 1 {
                return true;
            }
        }
        false
    }

    pub fn contains_duplicate3(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return false;
        }

        for i in 0..nums.len() - 1 {
            for j in (i + 1)..nums.len() {
                if nums[i] == nums[j] {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run1() {
        assert_eq!(Solution::contains_duplicate1(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::contains_duplicate1(vec![1, 2, 3, 4]), false);
        assert_eq!(
            Solution::contains_duplicate1(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }

    #[test]
    fn run2() {
        assert_eq!(Solution::contains_duplicate2(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::contains_duplicate2(vec![1, 2, 3, 4]), false);
        assert_eq!(
            Solution::contains_duplicate2(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }

    #[test]
    fn run3() {
        assert_eq!(Solution::contains_duplicate3(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::contains_duplicate3(vec![1, 2, 3, 4]), false);
        assert_eq!(
            Solution::contains_duplicate3(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }
}

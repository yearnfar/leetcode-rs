#![allow(dead_code)]
use std::i32;

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        } else if x < 10 {
            return true;
        } else if x % 10 == 0 {
            return false;
        }

        let mut x1 = x;
        let mut half = 0;

        while x1 > half {
            half = half * 10 + x1 % 10;
            x1 /= 10;
        }

        x1 == half || x1 == half / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run1() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}

#![allow(dead_code)]
use std::i32;

pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x: i64 = x as i64;
        let mut y: i64 = 0;
        let max = std::i32::MAX as i64;
        let min = std::i32::MIN as i64;

        while x != 0 {
            y = y * 10 + x % 10;
            if y > max || y < min {
                return 0;
            }

            x /= 10;
        }

        let y = y as i32;
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run1() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(1534236469), 0);
    }
}

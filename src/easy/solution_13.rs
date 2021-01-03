#![allow(dead_code)]

pub struct Solution;

// 罗马数字包含以下七种字符: I， V， X， L，C，D 和 M。

// 字符          数值
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000

// 例如， 罗马数字 2 写做 II ，即为两个并列的 1。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V + II 。

// 通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：

//     I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
//     X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。
//     C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。

// 给定一个罗马数字，将其转换成整数。输入确保在 1 到 3999 的范围内。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/roman-to-integer
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let f = |c: char| -> i32 {
            if c == 'I' {
                1
            } else if c == 'V' {
                5
            } else if c == 'X' {
                10
            } else if c == 'L' {
                50
            } else if c == 'C' {
                100
            } else if c == 'D' {
                500
            } else if c == 'M' {
                1000
            } else {
                0
            }
        };

        let chars: Vec<char> = s.to_string().chars().collect();
        let mut t = 0;
        let mut last = 0;

        for i in 0..chars.len() {
            if i == chars.len() - 1 || last < 0 {
                last = f(chars[i]);
            } else {
                last = f(chars[i]);
                if f(chars[i + 1]) > last {
                    last = 0 - last;
                }
            }
            t += last;
        }
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run1() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}

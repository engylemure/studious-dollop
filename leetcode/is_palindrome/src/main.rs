/**
 * https://leetcode.com/problems/valid-palindrome/
 */
fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            if !s[left].is_ascii_alphanumeric() {
                left += 1;
                continue;
            } else if !s[right].is_ascii_alphanumeric() {
                right -= 1;
                continue;
            }
            if s[right].to_ascii_lowercase() != s[left].to_ascii_lowercase() {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_solution(s: &str, expected: bool) {
        assert_eq!(Solution::is_palindrome(s.into()), expected);
    }
    #[test]
    fn hello() {
        assert_solution("hello", false);
    }

    #[test]
    fn hannah() {
        assert_solution("hannah", true);
    }
    
    #[test]
    fn zeroP() {
        assert_solution("0P", false);
    }
}

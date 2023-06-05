/**
 * https://leetcode.com/problems/reverse-string/
 */
fn main() {
    println!("reverse_string");
}

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();
        let n = len - 1;
        for i in 0..len / 2 {
            let buf = s[i];
            s[i] = s[n - i];
            s[n - i] = buf;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_solution(s: &str, expected: &str) {
        let mut chars = s.chars().collect();
        Solution::reverse_string(&mut chars);
        assert_eq!(chars, expected.chars().collect::<Vec<_>>());
    }
    #[test]
    fn hello() {
        assert_solution("hello", "olleh");
    }

    #[test]
    fn hannah() {
        assert_solution("hannah", "hannah");
    }
}

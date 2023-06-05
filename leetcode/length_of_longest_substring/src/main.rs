use std::collections::HashMap;

/**
 * https://leetcode.com/problems/longest-substring-without-repeating-characters/
 */
fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_map: HashMap<char, usize> = HashMap::new();
        let mut longest_substring_length = 0;
        let mut left = 0;
        let mut right = None;
        for (i, c) in s.chars().enumerate() {
            if let Some(idx) = char_map.get(&c) {
                if *idx >= left {
                    let substring_length: usize = right.unwrap() - left + 1;
                    if substring_length > longest_substring_length {
                        longest_substring_length = substring_length;
                    }
                    left = *idx + 1;
                }
            }
            char_map.insert(c, i);
            right = Some(i);
        }
        if let Some(right) = right {
            let substring_length: usize = right - left + 1;
            if substring_length > longest_substring_length {
                longest_substring_length = substring_length;
            }
        }
        longest_substring_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_solution(s: &str, expected: i32) {
        assert_eq!(Solution::length_of_longest_substring(s.into()), expected);
    }

    #[test]
    fn first() {
        assert_solution("abcabcbb", 3);
    }

    #[test]
    fn second() {
        assert_solution("bbbbbb", 1);
    }

    #[test]
    fn third() {
        assert_solution("pwwkew", 3);
    }

    #[test]
    fn fourth() {
        assert_solution("aab", 2);
    }

    #[test]
    fn fifth() {
        assert_solution("dvdf", 3);
    }
}
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

struct Solution;

fn is_vowel(c: &char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let chars: Vec<_> = s.chars().collect();
        let mut nVowels = chars[0..(k as usize)]
            .iter()
            .filter(|c| is_vowel(*c))
            .count();
        let mut maxVowels = nVowels;
        for i in 0..(s.len() - k as usize) {
            match (is_vowel(&chars[i]), is_vowel(&chars[i + k as usize])) {
                (true, false) => { nVowels -= 1; },
                (false, true) => { 
                    nVowels += 1;
                    if nVowels > maxVowels {
                        maxVowels = nVowels;
                    }
                },
                _ => {}
            }
        }
        maxVowels as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    fn assert_solution(s: &str, k: i32, expected: i32) {
        assert_eq!(Solution::max_vowels(s.into(), k), expected);
    }

    #[test]
    fn first() {
        assert_solution("abciiidef", 3, 3);
    }

    #[test]
    fn second() {
        assert_solution("aeiou", 2, 2);
    }

    #[test]
    fn third() {
        assert_solution("leetcode", 3, 2);
    }
}

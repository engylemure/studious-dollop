/**
 * https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
 */
fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;
        while left < right {
            let sum = numbers[left] + numbers[right];
            if sum == target {
                return vec![(left + 1) as i32, (right + 1) as i32]
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_solution(numbers: Vec<i32>, target: i32, expected: Vec<i32>) {
        assert_eq!(Solution::two_sum(numbers, target), expected);
    }
    
    #[test]
    fn first() {
        assert_solution(vec![2,7,11,15], 9, vec![1, 2]);
    }

    #[test]
    fn second() {
        assert_solution(vec![-1, 0], -1, vec![1, 2])
    }
}

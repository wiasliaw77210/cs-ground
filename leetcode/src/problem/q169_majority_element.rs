pub struct Solution;

impl Solution {
    // The solution is to get the most number in vector.
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut major: i32 = nums[0];
        let mut count: i32 = 1;
        for n in &nums[1..] {
            if count == 0 {
                count += 1;
                major = *n;
            } else if major == *n {
                count += 1;
            } else {
                count -= 1;
            }
        }
        return major;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element() {
        let cases: Vec<i32> = vec![3, 3, 2];
        assert_eq!(3, Solution::majority_element(cases));
    }
}

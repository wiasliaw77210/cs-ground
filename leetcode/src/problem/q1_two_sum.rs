use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mp: HashMap<i32, i32> = HashMap::new();
        let mut vecc: Vec<i32> = Vec::new();
        for (k_n, v_n) in nums.iter().enumerate() {
            if let Some((need, k)) = mp.get_key_value(&v_n) {
                vecc.extend_from_slice(&[*k, k_n as i32]);
                break;
            } else {
                mp.insert(target - v_n, k_n as i32);
            }
        };
        vecc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let n: Vec<i32> = vec![2, 7, 11, 15];
        let t = 9;
        let a: Vec<i32> = vec![0, 1];
        assert_eq!(a, Solution::two_sum(n, t));
    }
}

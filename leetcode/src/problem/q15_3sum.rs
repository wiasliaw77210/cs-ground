use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 { return Vec::new(); }
        let mut nv = nums.clone();
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let mut set: HashSet<Vec<i32>> = HashSet::new();
        nv.sort();
        // loop a
        let mut k_a: usize = 0;
        for v_a in &(nv.as_slice())[0..nv.len()-2] {
            k_a += 1;
            // loop b
            let mut k_b: usize = k_a;
            for v_b in &(nv.as_slice())[k_b..nv.len()-1] {
                k_b += 1;
                // loop c
                let k_c: usize = k_b;
                for v_c in &(nv.as_slice())[k_c..nv.len()] {
                    if v_a + v_b + v_c == 0 {
                        set.insert(vec![*v_a, *v_b, *v_c]);
                    }
                }
            }
        }
        for v in set.iter() {
            ret.push((*v.to_owned()).to_vec());
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        let case = vec![-1,0,1,2,-1,-4];
        let mut ans: Vec<Vec<i32>> = vec![
            vec![-1, -1, 2],
            vec![-1, 0, 1]
        ];
        assert_eq!(ans.sort(), Solution::three_sum(case).sort());
    }
}

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut ret: i32 = 0;
        let mut cv: Vec<char> = Vec::new();

        for c in s.chars() {
            let idx = cv.iter().rposition(|&x| x == c);
            cv = if let Some(tar) = idx {
                cv.iter().enumerate()
                    .filter(|&(i, _)| i > tar)
                    .map(|(_, e)| *e)
                    .collect()
            } else {
                cv
            };
            cv.push(c);
            let l = cv.len() as i32;
            ret = if l > ret { l } else { ret };
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sort() {
        let s: String = "abcabcbb".to_owned();
        assert_eq!(3, Solution::length_of_longest_substring(s));
    }
}

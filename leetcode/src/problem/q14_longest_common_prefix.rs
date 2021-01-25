pub struct Solution;

impl Solution {
    fn compare(s1: String, s2: String) -> String {
        let st1 = s1.chars();
        let st2 = s2.chars();
        let mut ret = String::new();
        for (c1, c2) in st1.zip(st2) {
            if c1 == c2 {
                ret.push(c1);
            } else {
                break;
            }
        }
        ret
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ret = String::new();
        let str_slice = strs.as_slice();
        if str_slice.len() == 0 {
            return ret;
        }
        ret = str_slice[0].to_owned();
        for s in &str_slice[1..] {
            ret = Self::compare(ret, s.to_owned());
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let v: Vec<String> = vec![];
        assert_eq!("".to_string(), Solution::longest_common_prefix(v));
        let v: Vec<String> = vec![
            "flower".to_owned(),
            "flow".to_owned(),
            "flight".to_owned(),
        ];
        assert_eq!("fl".to_string(), Solution::longest_common_prefix(v));
    }

    #[test]
    fn test_compare() {
        let s1 = String::from("flyw");
        let s2 = String::from("flower");
        assert_eq!("fl".to_string(), Solution::compare(s1, s2));
    }
}

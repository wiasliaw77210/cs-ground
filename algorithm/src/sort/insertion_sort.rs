pub fn sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        for j in 0..i {
            if arr[j] > arr[i] {
                arr.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut array: [i32; 6] = [5, 2, 4, 6, 1, 3];
        let ans: [i32; 6] = [1, 2, 3, 4, 5, 6];
        sort(&mut array);
        assert_eq!(ans, array);
    }
}

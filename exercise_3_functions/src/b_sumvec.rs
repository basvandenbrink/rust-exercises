// Uncomment the function and tests!

// // Returns the sum of given integer vector
// /
// / # Arguments
// /
// / * `v` - A vector containing integers
// 
pub fn sumvec(v: &Vec<i32>) -> i32 {
    let mut result : i32 = 0;
    for i in v {
        result += i;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::b_sumvec::sumvec;

    #[test]
    fn test_sumvec() {
        let result = sumvec(&vec![2, 3, 4]);
        assert_eq!(9, result);
    }
}

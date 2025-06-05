/*

    TEST FILE - DO NOT MODIFY

*/

use rstest::rstest;
use crate::task::{
    sort_vector,
    two_sum,
    prime_product
};

#[rstest]
#[case(vec![5, 4, 3, 2, 1])]
#[case(vec![4, 3, 7, 4, 9, 1, 2, 5])]
#[case(vec![-10, -16, 55, 42, -23, 45, 99])]
#[case(vec![55, 234, -2, 65, -65, 0, 44])]
#[case(vec![12, 9, 3, 14, 7, -10, -12, 32, 4])]
#[case(vec![1, 1, 1, 1, 0, 0, 0, 0])]
pub fn test_sort_vector(#[case] mut nums: Vec<i32>) {
    fn is_sorted(nums: Vec<i32>) -> bool {
    for i in 1..nums.len() {
        if nums[i] < nums[i - 1] {
            return false;
        } 
    }
    true
}
    sort_vector(&mut nums);
    assert!(is_sorted(nums));
}

#[rstest]
#[case(vec![-23, 19, 7, -12, 16], 7, vec![(3, 1)])]
#[case(vec![10, 16, -9, 4, 8, 3, -2, -15], 1, vec![(2, 0), (6, 5), (7, 1)])]
#[case(vec![-17, 44, 32, -3, -64], -20, vec![(3, 0), (4, 1)])]
#[case(vec![-16, 32, 8, 8, 24, 5, -8, -24], 16, vec![(1, 0), (3, 2), (6, 4)])]
#[case(vec![25, 17, 29, -20, 45, -60, 12], -31, vec![(5, 2)])]
#[case(vec![55, 105, -55, 45, 0, 100, 27], 100, vec![(3, 0), (5, 4)])]
pub fn test_two_sum(#[case] nums: Vec<i32>, #[case] k: i32, #[case] correct: Vec<(usize, usize)>) {
    fn is_correct_two_sum(output: &Vec<(usize, usize)>, correct: &Vec<(usize, usize)>) -> bool {
    for item in correct { // ensure minimum requirements
        if !output.contains(item) && !output.contains(&(item.0, item.1)) {
            return false;
        } 
    }
    for item in output { // ensure no extras
        if !correct.contains(item) && !correct.contains(&(item.0, item.1)) {
            return false;
        } 
    }
    true
}
    assert!(is_correct_two_sum(&two_sum(&nums, k), &correct));
}

#[rstest]
#[case(vec![2, 3, 5, 7, 11], 2310)]
#[case(vec![44, 15, 17, 90, 81, 31, 101], 53227)]
#[case(vec![41, 43, 16, 32, 39, 26, 83, 67], 9804043)]
#[case(vec![16, 65, 63, 13, 47, 19, 2, 14, 27, 39, 31], 719758)]
#[case(vec![3, 6, 9, 12, 15, 17, 19, 89, 97], 8365377)]
#[case(vec![53, 59, 60, 61, 67, 100, 101], 1290784949)]
pub fn test_prime_product(#[case] nums: Vec<u64>, #[case] output: u64) {
    assert_eq!(prime_product(&nums), output);
}
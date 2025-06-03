/*

    TEST FILE - DO NOT MODIFY

*/

use rstest::rstest;
use crate::task::{final_digit_sum, odd_even_product_sum, two_sum_lite};

#[rstest]
#[case(275, 5)]
#[case(12789, 9)]
#[case(999999999, 9)]
#[case(2782143, 9)]
#[case(234, 9)]
#[case(3245, 5)]
#[case(900345, 3)]
#[case(567, 9)]
#[case(7349, 5)]
#[case(45345, 3)]
#[case(65436, 6)]
#[case(42345, 9)]
#[case(7543, 1)]
pub fn test_final_digit_sum(#[case] num: u32, #[case] output: u32) {
    assert_eq!(final_digit_sum(num), output);
}

#[rstest]
#[case(1, -1, 3, 0, true)]
#[case(1, 2, 3, 2, false)]
#[case(28, -17, 21, 4, true)]
#[case(4, -5, 2, -1, true)]
#[case(7, 7, 7, 15, false)]
#[case(1, -1, 1, 1, false)]
#[case(81, -27, 45, 54, true)]
#[case(8, -16, -8, -24, true)]
#[case(0, 55, 33, 55, true)]
#[case(-3, -17, 25, 20, false)]
#[case(44, 44, 44, -88, false)]
pub fn test_two_sum_lite(#[case] a: i32, #[case] b: i32, #[case] c: i32, #[case] k: i32, #[case] output: bool) {
    assert_eq!(two_sum_lite(a, b, c, k), output);
}

#[rstest]
#[case(5, 2)]
#[case(1, -1)]
#[case(7, 8)]
#[case(11, 346)]
#[case(13, 3217)]
#[case(6, 1)]
#[case(19, 7274767)]
#[case(16, 28153)]
#[case(4, 0)]
#[case(9, 47)]
pub fn test_odd_even_product_sum(#[case] num: u32, #[case] output: i32) {
    assert_eq!(odd_even_product_sum(num), output);
} 
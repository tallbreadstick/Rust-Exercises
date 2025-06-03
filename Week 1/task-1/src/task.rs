/*

    1 - FINAL DIGIT SUM

    Given an unsigned integer [num], compute the sum of all digits. Compute again the sum of digits
    of the computed sum, repeating the process until a single digit final answer is reached.

    Test Case 1

    num = 275
    2 + 7 + 5 = 14
    1 + 4 = 5
    output = 5

    Test Case 2

    num = 12789
    1 + 2 + 7 + 8 + 9 = 27
    2 + 7 = 9
    output = 9

    Test Case 3

    num = 999999999
    9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 = 81
    8 + 1 = 9
    output = 9

*/

pub fn final_digit_sum(num: u32) -> u32 {
    todo!()
}

/*

    2 - TWO SUM LITE

    Given three signed integers [a], [b], and [c], return true if any two of the given integers
    added together can produce a sum of [k].

    Test Case 1

    a = 1, b = -1, c = 3, k = 0
    a(1) + b(-1) = k(0)
    output = true

    Test Case 2

    a = 1, b = 2, c = 3, k = 2
    no pair sum equals k(2)
    output = false

    Test Case 3

    a = 28, b = -17, c = 21, k = 4
    b(-17) + c(21) = k(4)
    output = true

*/

pub fn two_sum_lite(a: i32, b: i32, c: i32, k: i32) -> bool {
    todo!()
}

/*

    3 - ODD EVEN PRODUCT SUM

    Iterate over every integer from 1 to [num], getting the product of all odd numbers and the sum
    of all even numbers. Return the product divided by the sum, or -1 if the sum is zero.

    Hint: You can type cast using the 'as' keyword (i.e. 'num as i32')

    Test Case 1
    num = 5
    1 * 3 * 5 = 15
    2 + 4 = 6
    15 / 6 = 2
    output = 2

    Test Case 2
    num = 1
    output = -1

    Test Case 3
    num = 7
    1 * 3 * 5 * 7 = 105
    2 + 4 + 6 = 12
    105 / 12 = 8
    output = 8

*/

pub fn odd_even_product_sum(num: u32) -> i32 {
    todo!()
}
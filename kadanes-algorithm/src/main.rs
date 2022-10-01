/*
* Given an array containing both negative and positive integers. Find the
* contiguous sub-array with maximum sum.  
*
* INPUT: the first line of input contains an integer T denoting the
* number of test cases. The description of T test cases follows.
* The first line of each test case contains a single integer N denoting
* the size of array. The second line contains N space-separated integers
* A1, A2, ..., A_N denoting the elements of the array.
*
* OUTPUT: print the maximum sum of the contiguous sub-array in a separate
* line for each test case.
*/
#![allow(non_snake_case)]
mod solution_optimal;
mod solution_quadratic;
use solution_optimal::solution_optimal;
use solution_quadratic::solution_quadratic;

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];

    let result = solution_quadratic(nums);
    
    println!("Result: {}", result);
}

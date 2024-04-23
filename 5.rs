// Given a sorted array of integers, implement a function that returns the median of the array.
use std::io;

fn median(nums: &[i32]) -> f64 {
    let len = nums.len();
    if len % 2 == 1 {
        nums[len / 2] as f64
    } else {
        (nums[len / 2 - 1] as f64 + nums[len / 2] as f64) / 2.0
    }
}

fn main() {
    let numbers = vec![1, 3, 3, 6, 7, 8, 9];  
    println!("The median is {}", median(&numbers));
}

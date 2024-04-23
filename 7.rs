// Implement a function that returns the kth smallest element in a given array.


use std::io;

fn kth_smallest(mut nums: Vec<i32>, k: usize) -> i32 {
    nums.sort();
    nums[k - 1]
}

fn main() {
    let numbers = vec![7, 10, 4, 3, 20, 15];  // Example array
    let k = 3;  // Example position
    println!("The {}th smallest element is {}.", k, kth_smallest(numbers, k));
}

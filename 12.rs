// Find the maximum subarray sum in Rust

fn max_subarray_sum(nums: &[i32]) -> i32 {
    let (mut max_ending_here, mut max_so_far) = (0, i32::MIN);
    for &x in nums {
        max_ending_here = std::cmp::max(x, max_ending_here + x);
        max_so_far = std::cmp::max(max_so_far, max_ending_here);
    }
    max_so_far
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Maximum subarray sum is {}", max_subarray_sum(&nums));
}

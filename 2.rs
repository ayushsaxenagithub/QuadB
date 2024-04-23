// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = nums.len() as isize - 1;
    let mut result = None;

    while low <= high {
        let mid = low + (high - low) / 2;
        if nums[mid as usize] == target {
            result = Some(mid as usize);
            high = mid - 1;
        } else if nums[mid as usize] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}

fn main() {
    let numbers = vec![1, 2, 2, 3, 4, 5, 6];
    let mut input = String::new();
    println!("Enter the number:");
    std::io::stdin().read_line(&mut input).unwrap();
    let target: i32 = input.trim().parse().expect("Please type a number!");

    match binary_search(&numbers, target) {
        Some(index) => println!("First occurrence of number {} is at index {}.", target, index),
        None => println!("Number {} not found in the array.", target),
    }
}

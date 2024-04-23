// Merge two sorted arrays in Rust


// Time Complexity: O( nlog(n) + mlog(m) ) 

use std::collections::HashMap;

fn merge_arrays(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();

    // Insert elements from the first array into the map
    for num in a {
        *map.entry(num).or_insert(0) += 1;
    }

    // Insert elements from the second array into the map
    for num in b {
        *map.entry(num).or_insert(0) += 1;
    }

    // Collect keys from the map and sort them, then expand by frequency
    let mut result: Vec<i32> = Vec::new();
    let mut keys: Vec<_> = map.keys().collect();
    keys.sort(); 

    for key in keys {
        let count = map[key];
        for _ in 0..count {
            result.push(*key);
        }
    }

    result
}

fn main() {
    let a = vec![1, 3, 5, 7];
    let b = vec![2, 4, 6, 8];
    let merged = merge_arrays(a, b);
    println!("Merged and sorted array: {:?}", merged);
}


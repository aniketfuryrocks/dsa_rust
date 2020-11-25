use std::collections::HashMap;

/// LeetCode question https://leetcode.com/problems/subarray-sum-equals-k/
// Copy to playground from here

/// Brute force approach
/// O(n^2)
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut possibilities = 0;
    for i in 0..nums.len() {
        let mut sum = 0;
        for i in i..nums.len() {
            sum += nums[i];
            if sum == k {
                possibilities += 1;
            }
        }
    }
    return possibilities;
}

//O(n)
// Status: Not working 
pub fn subarray_sum_using_hash_map(nums: Vec<i32>, k: i32) -> i32 {
    let mut sum = 0;
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        sum += nums[i];
        *map.entry(sum).or_insert(0) += 1;
    }
    return *map.entry(k).or_insert(0);
}

#[test]
fn test_naive() {
    assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
    assert_eq!(subarray_sum(vec![1, 1, 3], 3), 2);
    assert_eq!(subarray_sum(vec![1, -1, 0], 0), 3);
}

#[test]
fn test_hash_map() {
    assert_eq!(subarray_sum(vec![3, 4, 7, 2, -3, 1, 4, 2], 0), 4);
    assert_eq!(subarray_sum_using_hash_map(vec![1, 1, 1], 2), 2);
    assert_eq!(subarray_sum_using_hash_map(vec![1, 1, 3], 3), 2);
    assert_eq!(subarray_sum_using_hash_map(vec![1, -1, 0], 0), 3);
}
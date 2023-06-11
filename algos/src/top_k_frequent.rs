use std::collections::HashMap;

/// O(n) space and time
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::<i32, usize>::default();
    let mut max_occurance = 0usize;

    for num in nums {
        let occ = *map.entry(num).and_modify(|k| *k += 1).or_insert(1);
        if occ > max_occurance {
            max_occurance = occ;
        }
    }

    let mut bucket: Vec<Vec<i32>> = vec![vec![]; max_occurance];

    for (num, repeat) in map {
        bucket[repeat - 1].push(num);
    }

    bucket.reverse();

    bucket
        .into_iter()
        .flatten()
        .take(k as usize)
        .collect()
}

#[test]
fn test() {
    assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    assert_eq!(top_k_frequent(vec![4,1,-1,2,-1,2,3], 2), vec![-1, 2]);
}

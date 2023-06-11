use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::<i32, i32>::default();

    for num in nums {
        *map.entry(num).or_default() += 1;
    }

    map.into_iter().sort();

    todo!()
}

#[test]
fn test() {
    top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
}

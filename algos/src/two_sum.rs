use std::vec;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // slow approach search for result O(n^2)
    // fast approach use a hashSet. Add every element of the vec into the hashSet
    // iterate. Subtract target and check if the result is in the hashset
    // use a hashMap instead to store the indices

    let mut map = std::collections::HashMap::<i32, (i32, i32)>::new();

    for (index, num) in nums.iter().enumerate() {
        let index  = index as i32;
        map.entry(*num).and_modify(|v| v.1 = index).or_insert((index, -1));
    }

    for (a, (index_1, index_2)) in map.iter() {
        let b = target - *a;

        if *a == b {
            if *index_2 == -1 {
                continue;
            }

            return vec![*index_1, *index_2]
        }

        if let Some((index_b, index_b_2)) = map.get(&b) {
           return vec![*index_1, *index_b];
        }
    }

    vec![]
}

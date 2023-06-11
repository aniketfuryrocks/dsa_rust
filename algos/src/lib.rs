pub mod decode_string;
pub mod max_sub_array;
pub mod subarray_sum_equals_k;
pub mod generate_parens;
pub mod top_k_frequent;
pub mod two_sum;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

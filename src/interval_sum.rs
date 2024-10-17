// 单纯区间和，不涉及修改
pub fn pure_interval_sum(prefix_sum_vec: &Vec<i32>, s: usize, e: usize) -> i32 {
    prefix_sum_vec[e] - prefix_sum_vec[s - 1]
}

pub fn prefix_sum(v: Vec<i32>) -> Vec<i32> {
    let mut prefix_sum_vec = Vec::with_capacity(v.len());
    let mut tmp = 0;
    for ele in v {
        tmp += ele;
        prefix_sum_vec.push(tmp);
    }
    prefix_sum_vec
}

#[cfg(test)]
mod interval_sum_test {
    use crate::interval_sum::pure_interval_sum;

    use super::prefix_sum;

    #[test]
    fn pure_interval_sum_test_1() {
        let v = vec![8, 13, 17, 9, 6, 8, 14, 11];
        let prefix_sum_vec = prefix_sum(v);
        assert_eq!(40, pure_interval_sum(&prefix_sum_vec, 2, 5));
    }
}
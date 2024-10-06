use std::usize;

/// 基本差分

/// https://www.luogu.com.cn/problem/P2367
pub fn minimal_score(org: &[isize], range_modify: &[(usize, usize, isize)]) -> isize {
    // 构建差分数组
    let mut diff_arr = vec![org[0]; org.len()];
    for i in 1..org.len() {
        diff_arr[i] = org[i] - org[i - 1];
    }

    for (l, r, w) in range_modify {
        diff_arr[*l] += w;
        if *r < diff_arr.len() - 1 {
            diff_arr[*r] -= w;
        }
    }

    let mut min = diff_arr[0];
    let mut cur = diff_arr[0];
    for i in 1..diff_arr.len() {
        cur += diff_arr[i];
        if min > cur {min = cur}
    }

    min
}

/// https://www.luogu.com.cn/problem/P3128
pub fn max_flow(n: usize, pipes: &[(usize, usize)], paths: &[(usize, usize)]) -> isize {
    todo!()    
}

#[cfg(test)]
mod basic_difference_tests {
    use crate::basic_difference::minimal_score;

    #[test]
    fn minimal_score_test_1() {
        let org = [1, 1, 1];
        let range_modify = [(0, 1, 1), (1, 2, -3)];
        assert_eq!(-2, minimal_score(&org, &range_modify));
    }
}
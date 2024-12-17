// 算法导论21-1(脱机最小值)
// n: 数据范围[1, n];
// m: extract_min执行次数;
// seq：insert和extract_min执行序列, 例: ['3', '7', 'E', '5'], 代表操作序列insert(3), insert(7), extract_min(), insert(5)
pub fn offline_minimum(n: usize, m: usize, seq: Vec<char>) -> Vec<usize> {
    let mut res = vec![usize::MAX; m + 1];

    // 每一个元素代表(所属集合的父集合)
    let mut k: Vec<usize> = (0..m + 2).collect();
    let mut ele = vec![usize::MAX; n + 1];

    let mut i = 1;
    for c in seq {
        if c != 'E' {
            ele[(c as u8 - b'0') as usize] = i;
        } else {
            i += 1;
        }
    }

    for i in 1..n {
        let real_k;
        if k[ele[i]] == ele[i] {
            real_k = ele[i];
        } else {
            real_k = find_real_k(&mut k, ele[i]);
        }
        if real_k != m + 1 {
            res[real_k] = i;

            k[real_k] = find_real_k(&mut k, real_k + 1);
        }
    }

    res
}

/**
 * 找到k[idx]所属的顶级集合(带路径压缩)
 * k: 数组元素的值代表所属集合, 如果k[i] == i, 表示其是顶级集合. 例如: k[1] = 2, k[2] = 3, k[3] = 3, 表示集合k[1]和k[2]都属于集合k[3]
 * idx: k[idx]
 */
fn find_real_k(k: &mut Vec<usize>, idx: usize) -> usize {
    let mut stack = Vec::new();
    let mut tmp_idx = idx;
    while k[tmp_idx] != tmp_idx {
        stack.push(tmp_idx);
        tmp_idx = k[tmp_idx];
    }

    while !stack.is_empty() {
        k[stack.pop().unwrap()] = k[tmp_idx];
    }

    k[tmp_idx]
}

#[cfg(test)]
mod offline_minimum_tests {
    use super::offline_minimum;

    #[test]
    fn offline_minimum_test1() {
        let mut seq = vec!['4', '8', 'E', '3', 'E', '9', '2', '6', 'E', 'E', 'E', '1', '7', 'E', '5'];
        let res = offline_minimum(9, 6, seq);
        println!("{:?}", res)
    }
}
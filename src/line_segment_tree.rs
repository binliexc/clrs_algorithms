use std::usize;

/// 线段树
pub struct LineSegmentTree {
    nodes: Vec<usize>,

    m: usize,
}

impl LineSegmentTree {
    pub fn new(elements: Vec<usize>) -> Self {
        let mut nodes = vec![0; closest_second_power(elements.len()) << 1];
        let m = Self::get_m(elements.len());

        let mut idx = m;
        for element in elements {
            nodes[idx] = element;
            idx += 1;
        }

        idx = m - 1;
        while idx > 0 {
            nodes[idx] = nodes[idx << 1] + nodes[(idx << 1) + 1];
            idx -= 1;
        }

        LineSegmentTree { nodes, m }
    }

    pub fn query(&self, mut s: usize, mut t: usize) -> usize {
        s = s + self.m - 1;
        t = t + self.m + 1;
        println!("{s}, {t}");

        let mut ans = 0;

        while s ^ t != 1 {
            if s & 1 == 0 {
                println!("{}", s ^ 1);
                ans += self.nodes[s ^ 1];
            }
            if t & 1 == 1 {
                ans += self.nodes[t ^ 1];
            }

            s >>= 1;
            t >>= 1;
        }

        ans
    }

    pub fn change(&mut self, mut n: usize, new_val: usize) {
        n += self.m;
        self.nodes[n] = new_val;
        n >>= 1;

        while n >= 1 {
            self.nodes[n] = self.nodes[n + n] + self.nodes[n + n + 1];
            n >>= 1;
        }
    }

    fn get_m(len: usize) -> usize {
        let len_f64 = len as f64;
        1 << len_f64.log2().floor() as usize
    }
}

/// 找到和n最接近的大于等于n的二次幂
pub fn closest_second_power(mut n: usize) -> usize {
    n -= 1;
    n = n | (n >> 1);
    n = n | (n >> 2);
    n = n | (n >> 4);
    n = n | (n >> 8);
    n = n | (n >> 16);
    n = n | (n >> 32);
    n + 1
}

#[cfg(test)]
mod line_segment_tree_tests {
    use crate::line_segment_tree::closest_second_power;

    #[test]
    fn closest_second_power_test_1() {
        assert_eq!(4, closest_second_power(4));
        assert_eq!(16, closest_second_power(11));
    }
}
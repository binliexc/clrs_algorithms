use std::{
    cmp::{max, min},
    usize,
};

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
        len >> 1
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

// 支持区间修改, RMQ的线段树
#[derive(Debug)]
pub struct LineSegmentTreeRmq {
    t: Vec<isize>,

    m: usize,
}

impl LineSegmentTreeRmq {
    pub fn new(elements: Vec<isize>) -> Self {
        let mut t = vec![0; closest_second_power(elements.len()) << 1];
        let m = t.len() >> 1;

        let mut i = 0;
        while i < elements.len() {
            t[i + m] = elements[i];
            if i + m + 1 < elements.len() {
                t[i + m + 1] = elements[i + 1];
            }
            i += 1;
        }

        i = m - 1;
        while i > 0 {
            let maximal;

            if t[i << 1] < t[i << 1 ^ 1] {
                maximal = t[i << 1 ^ 1];
            } else {
                maximal = t[i << 1];
            }

            t[i] = maximal;
            t[i << 1] -= maximal;
            t[i << 1 ^ 1] -= maximal;

            i -= 1;
        }

        LineSegmentTreeRmq { t, m }
    }

    pub fn add_x(&mut self, mut s: usize, mut e: usize, x: isize) {
        if s > 0 && e < self.t.len() - 1 {
            s = s + self.m - 1;
            e = e + self.m + 1;

            while s ^ e != 1 {
                self.t[s ^ 1] += x;
                self.t[e ^ 1] += x;

                let a = min(self.t[s], self.t[s ^ 1]);
                self.t[s] -= a;
                self.t[s ^ 1] -= a;
                self.t[s >> 1] += a;

                let b = min(self.t[s], self.t[s ^ 1]);
                self.t[s] -= b;
                self.t[s ^ 1] -= b;
                self.t[s >> 1] += b;

                s >>= 1;
                e >>= 1;
            }
        }
    }

    pub fn max(&self, mut s: usize, mut e: usize) -> isize {
        let mut lans = 0;
        let mut rans = 0;

        s = s + self.m - 1;
        e = e + self.m + 1;

        let mut initialized = false;

        while s ^ e != 1 {
            if !initialized {
                if s & 1 == 0 {
                    lans += self.t[s ^ 1];
                } else {
                    lans += max(self.t[s], self.t[s ^ 1]);
                }

                if e & 1 == 1 {
                    rans += self.t[e ^ 1];
                } else {
                    rans += max(self.t[e], self.t[e ^ 1]);
                }

                initialized = true;
            } else {
                lans += self.t[s];
                rans += self.t[e];
            }
            if s & 1 == 0 {
                if lans < self.t[s ^ 1] {
                    lans += self.t[s ^ 1] - lans;
                }
            }
            if e & 1 == 1 {
                if rans < self.t[e ^ 1] {
                    rans += self.t[e ^ 1] - rans;
                }
            }

            s >>= 1;
            e >>= 1;
        }
        lans += self.t[s];
        rans += self.t[e];
        let mut ans = max(lans, rans);
        while s > 0 {
            ans += self.t[s];
            s >>= 1;
        }

        ans
    }
}

#[cfg(test)]
mod line_segment_tree_tests {
    use crate::line_segment_tree::closest_second_power;

    use super::LineSegmentTreeRmq;

    #[test]
    fn closest_second_power_test_1() {
        assert_eq!(4, closest_second_power(4));
        assert_eq!(16, closest_second_power(11));
    }

    #[test]
    fn line_segment_tree_rmq_test_1() {
        let elements = vec![8, 13, 17, 9, 6, 8, 14, 11];
        let rmq_tree = LineSegmentTreeRmq::new(elements);
        println!("{:?}", rmq_tree.t);
        assert_eq!(14, rmq_tree.max(3, 6));
    }
}

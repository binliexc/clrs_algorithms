use std::{
    cmp::{max, min}, f32::consts::E, usize
};

/// 线段树
pub struct LineSegmentTree {
    nodes: Vec<isize>,

    m: usize,
}

impl LineSegmentTree {
    pub fn new(elements: Vec<isize>) -> Self {
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

    pub fn query(&self, mut s: usize, mut t: usize) -> isize {
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

    pub fn change(&mut self, mut n: usize, new_val: isize) {
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

            let mut first_time = true;
            while s ^ e != 1 {
                if first_time {
                    if s ^ 1 == 1 {
                        self.t[s ^ 1] += x;
                    }
                    if e ^ 1 == 0 {
                        self.t[e ^ 1] += x;
                    }
                    first_time = false;
                } else {
                    self.t[s ^ 1] += x;
                    self.t[e ^ 1] += x;
                }

                let a = min(self.t[s], self.t[s ^ 1]);
                self.t[s] -= a;
                self.t[s ^ 1] -= a;
                self.t[s >> 1] += a;

                let b = min(self.t[e], self.t[e ^ 1]);
                self.t[e] -= b;
                self.t[e ^ 1] -= b;
                self.t[e >> 1] += b;

                s >>= 1;
                e >>= 1;
            }
            let min_s_e = min(self.t[s], self.t[e]);
            self.t[s] -= min_s_e;
            self.t[s ^ 1] -= min_s_e;
            self.t[s >> 1] += min_s_e;
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
        s >>= 1;

        while s > 0 {
            ans += self.t[s];
            s >>= 1;
        }

        ans
    }
}

// 支持单点查询，区间修改的区间树trait
pub trait LineSegmentTree3 {
    // 新建树的关联方法
    fn new_lst3(a: Vec<isize>) -> Self;

    // 原数组[s, e]区间的值都新增x
    fn interval_add_x(&mut self, s: usize, e: usize, x: isize);

    // 为用堆建树的节点i新增x
    fn add_x(&mut self, i: usize, x: isize);

    // 得到原数组下标为idx的值
    fn get_val(&self, idx: usize) -> isize;
}

impl LineSegmentTree3 for LineSegmentTree {
    fn new_lst3(a: Vec<isize>) -> Self {
        let m = closest_second_power(a.len());
        let mut t = vec![0; m << 1];
        t[m] = a[0];
        for i in 1..a.len() {
            t[m + i] = a[i] - a[i - 1];
        }
        for i in (1..m).rev() {
            t[i] = t[i << 1] + t[(i << 1) + 1];
        }
        LineSegmentTree {
            nodes: t,
            m: m,    
        }
    }

    fn interval_add_x(&mut self, s: usize, e: usize, x: isize) {
        self.add_x(s + self.m, x); 
        self.add_x(e + self.m + 1, -x);
    }

    fn add_x(&mut self, mut i: usize, x: isize) {
        while i > 0 {
            self.nodes[i] += x;
            i >>= 1;
        }
    }
    
    fn get_val(&self, idx: usize) -> isize {
        if idx == 1 {
            self.nodes[self.m >> 1]
        } else {
            let mut s = self.m;
            let mut e = self.m + idx + 1;

            let mut ans = 0;
            while s ^ e != 1 {
                if e & 1 == 1 {
                    ans += self.nodes[e ^ 1];
                }
                s >>= 1;
                e >>= 1;
            }
            ans += self.nodes[s];
            ans
        }
    }
}

#[cfg(test)]
mod line_segment_tree_tests {
    use std::env::temp_dir;

    use crate::line_segment_tree::closest_second_power;

    use super::{LineSegmentTree, LineSegmentTree3, LineSegmentTreeRmq};

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

    #[test]
    fn line_segment_tree_rmq_test_2() {
        let elements = vec![8, 13, 17, 9, 6, 8, 14, 11];
        let mut rmq_tree = LineSegmentTreeRmq::new(elements);
        rmq_tree.add_x(2, 5, 2);
        println!("{:?}", rmq_tree.t);
        assert_eq!(rmq_tree.max(2, 6), 19);
    }

    #[test]
    fn line_segment_tree_3_test_1() {
        let elements = vec![10, 19, 20, 1, 5, 9, 12, 7];
        let mut tree = LineSegmentTree::new_lst3(elements);
        println!("before add_x: {:?}", tree.nodes);
        tree.interval_add_x(2, 5, 4);
        println!("after add_x: {:?}", tree.nodes);
        assert_eq!(tree.get_val(2), 24);
        assert_eq!(tree.get_val(4), 9);
        assert_eq!(tree.get_val(5), 13);
    }
}

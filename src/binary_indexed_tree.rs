use std::process::id;

/// 树状数组
pub struct BinaryIndexedTree {
    // 树状数组
    c: Vec<isize>,

    // 原数组
    a: Vec<isize>,
}

impl BinaryIndexedTree {
    // 时间复杂度 nlgn 的初始化方法
    pub fn new(a: Vec<isize>) -> Self {
        let len = a.len() + 1;

        let mut tree = BinaryIndexedTree { 
            c: vec![0; len],
            a: vec![0; len],
        };

        for (index, value) in a.iter().enumerate() {
            tree.change(index + 1, *value);
        }

        tree
    }

    // a[1]..a[x]的和
    pub fn get_sum(self, mut x: usize) -> isize {
        let mut ans = 0;
        while x > 0 {
            ans += self.c[x];
            x -= lowbit(x.try_into().unwrap());
        }
        ans
    }

    // 将原数组中 `idx` 下标的值修改为 `new_val`
    pub fn change(&mut self, mut idx: usize, new_val: isize) {
        let diff = new_val - self.c[idx];
        self.a[idx] = new_val;
        while idx < self.c.len() {
            self.c[idx] += diff;
            idx += lowbit(idx.try_into().unwrap());
        }
    }
}

pub fn lowbit(x: isize) -> usize {
    (x & -x) as usize
}

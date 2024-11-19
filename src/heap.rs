pub struct MaxHeap {
    val: Vec<i32>,
}

impl MaxHeap {
    // 注意: 数组v的第一个元素是v[1], v[0]不参与构造
    pub fn new(mut v: Vec<i32>) -> Self {
        let mut p = (v.len() - 1) >> 1;

        while p > 0 {
            Self::max_heapify(&mut v, p);
            p -= 1;
        }

        MaxHeap { val: v }
    }

    fn max_heapify(v: &mut Vec<i32>, idx: usize) {
        assert!(idx < v.len(), "索引位置超过当前数组大小");
        let mut i = idx;
        while i << 1 < v.len() {
            let mut large_idx = i;
            let l = i << 1;
            let r = l + 1;
            if v[l] > v[large_idx] {
                large_idx = l;
            }
            if r < v.len() && v[r] > v[large_idx] {
                large_idx = r;
            }

            if large_idx == i {
                break;
            } else {
                let tmp = v[i];
                v[i] = v[large_idx];
                v[large_idx] = tmp;
                i = large_idx;
            }
        }
    }

    pub fn peek_max(&self) -> i32 {
        self.val[1]
    }

    pub fn extract_max(&mut self) -> Option<i32> {
        let len = self.val.len();
        if len == 1 {
            return None;
        } else if len == 2 {
            return self.val.pop();
        }

        let max = self.val[1];

        self.val[1] = self.val.pop().unwrap();
        Self::max_heapify(&mut self.val, 1);

        Some(max)
    }

    pub fn remove(&mut self, ele: i32) -> Result<(), String> {
        let mut idx = 0;
        for i in 1..self.val.len() {
            if self.val[i] == ele {
                idx = i;
                break;
            }
        }

        if idx == 0 {
            Err(format!("未在堆中找到值为{ele}的元素"))
        } else {
            let last = self.val.pop().unwrap();
            if idx != self.val.len() {
                if last < self.val[idx] {
                    self.val[idx] = last;
                    Self::max_heapify(&mut self.val, idx);
                } else {
                    while idx > 0 && self.val[idx >> 1] < self.val[idx] {
                        let tmp = self.val[idx >> 1];
                        self.val[idx >> 1] = self.val[idx];
                        self.val[idx] = tmp;

                        idx >>= 1
                    }
                }
            }
            Ok(())
        }
    }

    pub fn push(&mut self, ele: i32) {
        self.val.push(ele);

        let mut idx = self.val.len() - 1;
        while idx > 1 && self.val[idx >> 1] < self.val[idx] {
            let tmp = self.val[idx >> 1];
            self.val[idx >> 1] = self.val[idx];
            self.val[idx] = tmp;

            idx >>= 1;
        }
    }
}

#[cfg(test)]
mod heap_test {
    use std::i32;

    use rand::Rng;

    use super::MaxHeap;

    // 主要测试基本的构建堆和弹出最大值功能
    #[test]
    fn heap_test1() {
        let mut v = Vec::new();

        let mut rng = rand::thread_rng();
        v.push(i32::MIN);
        for _ in 0..19 {
            v.push(rng.gen_range(1..100));
        }

        let mut v1 = v.clone();
        v1.sort();
        println!("v1的值: {:?}", v1);
        let mut max_heap = MaxHeap::new(v);

        for i in (1..v1.len()).rev() {
            println!("第{:?}次, val: {:?}", v1.len() - i, max_heap.val);
            assert_eq!(max_heap.extract_max().unwrap(), v1[i]);
        }

        assert_eq!(max_heap.extract_max(), None);
    }

    // 测试remove方法
    #[test]
    fn heap_test2() {
        let v = vec![i32::MIN, 5, 8, 11, 320, 13, 57, 32, 99, 60, 77];
        let mut max_heap = MaxHeap::new(v);
        max_heap.remove(320);
        assert_eq!(max_heap.peek_max(), 99);
        max_heap.remove(57);
        println!("{:#?}", max_heap.val);
        assert_eq!(
            max_heap.remove(57),
            Err("未在堆中找到值为57的元素".to_string())
        );
    }

    // 测试push方法
    #[test]
    fn heap_test3() {
        let v = vec![i32::MIN, 5, 8, 11, 320, 13, 57, 32, 99, 60, 77];
        let mut max_heap = MaxHeap::new(v);
        max_heap.push(321);
        assert_eq!(max_heap.extract_max().unwrap(), 321);
        assert_eq!(max_heap.extract_max().unwrap(), 320);
    }
}

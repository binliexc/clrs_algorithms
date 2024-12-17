use std::{collections::HashMap, mem::ManuallyDrop, ptr, sync::OnceLock};

pub static mut NODE_CONTAINER: OnceLock<HashMap<isize, *mut Node>> = OnceLock::new();

// 不相交集合
#[derive(PartialEq, Debug)]
pub struct Set {
    head: *mut Node,
    tail: *mut Node,
    len: usize,
}

#[derive(Debug)]
pub struct Node {
    val: isize,
    next: Option<Box<Node>>,
    pub set: *mut Set,
}

impl Set {
    pub fn make_set(val: isize) -> *mut Set {
        let n = Box::into_raw(Box::new(Node {
            val: val,
            next: None,
            set: ptr::null_mut(),
        }));

        let set = Box::into_raw(Box::new(Set {
            head: n,
            tail: n,
            len: 1,
        }));

        let container = unsafe { NODE_CONTAINER.get_mut_or_init(|| HashMap::new()) };
        container.insert(val, n);

        let mut node = unsafe { Box::from_raw(n) };
        node.set = set;
        let _ = ManuallyDrop::new(node);

        set
    }

    pub fn union(a: Box<Set>, b: Box<Set>) {
        // 如果是同一个集合, 则直接返回(但不能让rust自动drop掉)
        if a == b {
            let _a = ManuallyDrop::new(a);
            let _b = ManuallyDrop::new(b);
            return;
        };

        let smaller: Box<Set>;
        let mut bigger: Box<Set>;
        if a.len < b.len {
            smaller = a;
            bigger = b;
        } else {
            smaller = b;
            bigger = a;
        }
        bigger.len += smaller.len;

        let mut bigger_tail = unsafe { Box::from_raw(bigger.tail) };
        let smaller_tmp = unsafe { Box::from_raw(smaller.head) };
        bigger_tail.next = Some(smaller_tmp);
        bigger.tail = smaller.tail;
        let bigger_ptr: *mut Set = Box::into_raw(bigger);

        let mut tmp = bigger_tail.next.as_deref_mut();
        while tmp.is_some() {
            let tmp_node = tmp.unwrap();
            tmp_node.set = bigger_ptr;

            tmp = tmp_node.next.as_deref_mut();
        }

        let _ = ManuallyDrop::new(bigger_tail);
    }

    pub fn find_set(val: isize) -> Box<Set> {
        let container = unsafe { NODE_CONTAINER.get().unwrap() };

        match container.get(&val) {
            Some(n) => {
                let node = unsafe { Box::from_raw(*n) };
                let set = unsafe { Box::from_raw(node.set) };
                let _ = ManuallyDrop::new(node);
                set
            }
            None => panic!("节点 {} 不存在", val),
        }
    }
}

#[cfg(test)]
mod disjoint_set_tests {
    use std::mem::ManuallyDrop;

    use super::{Set, NODE_CONTAINER};

    #[test]
    fn disjoint_set_test1() {
        let nodes = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let edges = vec![(1, 4), (5, 7), (1, 3), (8, 9), (1, 2), (5, 6), (2, 3)];

        for node in nodes {
            Set::make_set(node);
        }
        println!("make set complete");

        for (a, b) in edges {
            let set_a = Set::find_set(a);
            let set_b = Set::find_set(b);

            Set::union(set_a, set_b);
        }
        println!("union set complete");

        let container = unsafe { NODE_CONTAINER.get().unwrap() };
        for (key, val) in container {
            let n = unsafe { Box::from_raw(*val) };
            println!("key: {key}, set: {:p}", n.set);
            let _ = ManuallyDrop::new(n);
        }
    }
}

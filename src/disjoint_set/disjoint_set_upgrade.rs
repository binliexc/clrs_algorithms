use std::{mem::ManuallyDrop, ptr};

// 使用按秩合并(union by rank)和路径压缩(path compression)的不相交集合
pub struct Node {
    val: usize,
    p: *mut Node,
    rank: usize,
}

impl Node {
    pub fn make_set(val: usize) -> *mut Node {
        let ptr = Box::into_raw(Box::new(Node {
            val: val,
            p: ptr::null_mut(),
            rank: 0,
        }));

        let mut n = unsafe { Box::from_raw(ptr) };
        n.p = ptr;

        let _ = ManuallyDrop::new(n);
        ptr
    }

    pub fn find_set(ptr: *mut Node) -> *mut Node {
        let mut n = unsafe { Box::from_raw(ptr) };
        if n.p != ptr {
            n.p = Self::find_set(n.p);
        }
        let p_ptr = n.p;
        let _ = ManuallyDrop::new(n);
        p_ptr
    }

    pub fn union(a_ptr: *mut Node, b_ptr: *mut Node) {
        let a_set_ptr = Self::find_set(a_ptr);
        let b_set_ptr = Self::find_set(b_ptr);
        let mut a_set = unsafe { Box::from_raw(a_set_ptr) };
        let mut b_set = unsafe { Box::from_raw(b_set_ptr) };

        if a_set.rank == b_set.rank {
            b_set.p = a_set_ptr;
            a_set.rank += 1;
        } else if a_set.rank > b_set.rank {
            b_set.p = a_set_ptr;
        } else {
            a_set.p = b_set_ptr;
        }

        let _ = ManuallyDrop::new(a_set);
        let _ = ManuallyDrop::new(b_set);
    }
}

#[cfg(test)]
mod disjoint_set_upgrade_tests {
    use std::collections::HashMap;

    use crate::disjoint_set::disjoint_set_upgrade::Node;

    #[test]
    fn disjoint_set_upgrade_test1() {
        let mut node_container = HashMap::with_capacity(10);
        let nodes = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let edges = vec![(1, 4), (5, 7), (1, 3), (8, 9), (1, 2), (5, 6), (2, 3)];

        for node in nodes {
            node_container.insert(node, Node::make_set(node));
        }

        for (a, b) in edges {
            Node::union(
                *node_container.get(&a).unwrap(),
                *node_container.get(&b).unwrap(),
            );
        }

        let mut map = HashMap::new();
        for (key, val) in node_container {
            map.entry(Node::find_set(val)).or_insert_with(|| Vec::new()).push(key);
        }

        for (key, val) in map {
            println!("ptr: {:p}, vals: {:?}", key, val);
        }
    }
}

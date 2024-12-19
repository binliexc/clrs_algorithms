use std::{collections::VecDeque, ptr};

// 算法导论21-2(深度确定)
pub struct Container {
    pub nodes: Vec<Option<TreeNode>>,
}

#[derive(Debug)]
pub struct TreeNode {
    pub val: usize,
    pub p: *mut TreeNode,
    pub depth: usize,
}

impl Container {
    // 容器包含的节点值范围在[0, max]
    pub fn new(max: usize) -> Self {
        let v = (0..=max).map(|_| None).collect();
        Container { nodes: v }
    }

    pub fn make_tree(&mut self, val: usize) -> Result<(), String> {
        if self.nodes.len() <= val {
            return Err(format!("val: {val} 超出范围"));
        }
        self.nodes[val] = Some(TreeNode {
            val: val,
            p: ptr::null_mut(),
            depth: 0,
        });

        let node = self.nodes[val].as_mut().unwrap();
        node.p = node;

        Ok(())
    }

    pub fn find_depth(&mut self, val: usize) -> usize {
        match self.nodes.get_mut(val) {
            Some(n) => Self::find_node_depth(n.as_mut().unwrap()),
            None => panic!("val: {val} 对应的结点不存在"),
        }
    }

    fn find_node_depth(n: &mut TreeNode) -> usize {
        let mut v = VecDeque::new();
        let mut depth = 0;

        let mut tmp_ptr: *mut TreeNode = n;
        let mut tmp_node = n;
        while tmp_node.p != tmp_ptr {
            println!("node: {:?}", tmp_node);
            let ptr1: *mut TreeNode = tmp_node;
            println!("curptr: {:p}", ptr1);

            depth += tmp_node.depth;

            tmp_ptr = tmp_node.p;
            let tmp_n = unsafe { &mut *tmp_node.p };

            v.push_back(tmp_node);
            tmp_node = tmp_n;
        }

        let mut cur_depth = depth;
        while !v.is_empty() {
            let e = v.pop_front().unwrap();
            e.p = tmp_ptr;
            let tmp_depth = e.depth;
            e.depth = cur_depth;
            cur_depth -= tmp_depth;
        }

        depth
    }

    pub fn graft(&mut self, idx_r: usize, idx_v: usize) -> Result<(), String> {
        if idx_r >= self.nodes.len() {
            return Err(format!("idx_r: {idx_r} 对应的结点不存在"));
        } else if idx_v >= self.nodes.len() {
            return Err(format!("idx_v: {idx_v} 对应的结点不存在"));
        } else if let None = self.nodes[idx_r] {
            return Err(format!("idx_r: {idx_r} 对应的结点不存在"));
        } else if let None = self.nodes[idx_v] {
            return Err(format!("idx_v: {idx_v} 对应的结点不存在"));
        }

        let raw_r: *mut TreeNode = self.nodes[idx_r].as_mut().unwrap();
        let raw_v: *mut TreeNode = self.nodes[idx_v].as_mut().unwrap();

        Self::graft_node(unsafe { &mut *raw_r }, unsafe { &mut *raw_v });

        Ok(())
    }

    // 使结点r(假设它为一棵树的树根)成为结点v的孩子
    fn graft_node(r: &mut TreeNode, v: &mut TreeNode) {
        let v_ptr: *mut TreeNode = v;
        r.depth = 1;
        r.p = v_ptr;
    }
}
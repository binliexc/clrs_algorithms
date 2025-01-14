pub struct TreeNode {
    pub val: usize,
    pub children: Vec<*mut TreeNode>,
}

// Trajan 脱机最小公共祖先
// `root` 代表树的根节点, `range` 代表树的结点的值的范围在[0, range], `p` 代表想要获取最小公共结点的树结点对的集合
pub fn offline_lca(root: &TreeNode, range: usize, p: Vec<(usize, usize)>) -> Vec<(usize, usize, usize)> {
    let mut res = Vec::with_capacity(p.len());

    let mut disjoint_set = DisjointSet::new(range);
    let mut marks = vec![false; range + 1];
    let mut pairs = vec![Vec::new(); range + 1];
    for (a, b) in p {
        pairs[a].push(b);
        pairs[b].push(a);
    }
    
    lca(root, &mut marks, &pairs, &mut res, &mut disjoint_set);
    res
}

fn lca(n: &TreeNode, marks: &mut Vec<bool>, pairs: &Vec<Vec<usize>>, res: &mut Vec<(usize, usize, usize)>, disjoint_set: &mut DisjointSet) {
    disjoint_set.make_set(n.val);
    for ptr in &n.children {
        let child = unsafe { &**ptr };
        lca(child, marks, pairs, res, disjoint_set);
        disjoint_set.union(child.val, n.val);
    }
    marks[n.val] = true;
    for i in &pairs[n.val] {
        if marks[*i] {
            res.push((n.val, *i, disjoint_set.find_set(*i).unwrap()));
        }
    }
}

struct Node {
    p: usize,
    val: usize,
}

struct DisjointSet {
    containers: Vec<Option<Node>>,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        DisjointSet {
            containers: (0..=size).map(|_| None).collect(),
        }
    }

    pub fn make_set(&mut self, val: usize) -> Result<(), String> {
        if val > self.containers.len() {
            return Err(format!("val: {val}超过最大限制{}", self.containers.len()));
        }
        self.containers[val] = Some(Node { p: val, val });
        Ok(())
    }

    pub fn union(&mut self, u: usize, v: usize) -> Result<(), String> {
        if u > self.containers.len() {
            return Err(format!("u: {u}超过最大限制{}", self.containers.len()));
        } else if v > self.containers.len() {
            return Err(format!("v: {v}超过最大限制{}", self.containers.len()));
        } else if self.containers[u].is_none() {
            return Err(format!("u: {u}未初始化"));
        } else if self.containers[v].is_none() {
            return Err(format!("v: {v}未初始化"));
        }
        self.containers[u].as_mut().unwrap().p = v;
        Ok(())
    }

    pub fn find_set(&mut self, i: usize) -> Result<usize, String> {
        if i > self.containers.len() {
            return Err(format!("i: {i}超过最大限制{}", self.containers.len()));
        } else if self.containers[i].is_none() {
            return Err(format!("i: {i}未初始化"));
        }
        let n = self.containers[i].as_mut().unwrap();
        let mut cur_p = n.p;
        if cur_p != i {
            cur_p = self.find_set(cur_p)?;
        }
        self.containers[i].as_mut().unwrap().p = cur_p;
        Ok(cur_p)
    }
}
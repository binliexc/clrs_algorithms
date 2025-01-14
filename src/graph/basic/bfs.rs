pub struct AdjacencyList {
    // 保存结点信息
    nodes: Vec<Node>,
    // 邻接链表
    list: Vec<Vec<usize>>,
    // 结点范围
    range: usize,
}

struct Node {
    val: usize,
    // 前置结点
    pre: Option<usize>,
    // 颜色: 初始化的时候为'w'白色, 邻近结点有未访问的时候为'g'灰色, 所有邻近结点都被访问了为'b'黑色
    col: char,
}

impl AdjacencyList {
    // 初始化
    pub fn initialize(range: usize, edges: Vec<(usize, usize)>) -> Self {
        let mut nodes = Vec::with_capacity(range);
        let mut list = Vec::with_capacity(range);

        for i in 0..range {
            nodes.push(Node {
                val: i,
                pre: None,
                col: 'w',
            });

            list.push(Vec::new());
        }

        for (a, b) in edges {
            list[a].push(b);
            list[b].push(a);
        }

        AdjacencyList { nodes, list, range }
    }

    pub fn bfs(&mut self, start: usize) -> Vec<String> {
        if start >= self.range {
            panic!("start: {start} 超出范围");
        }

        let mut res = vec![String::new(); self.range];
        res[start] = start.to_string();

        let mut queue = Vec::new();
        queue.push(start);
        while !queue.is_empty() {
            let n = queue.pop().unwrap();
            for t in &self.list[n] {
                if self.nodes[*t].col == 'w' {
                    res[*t] = res[n].clone() + "_" + &*t.to_string();
                    self.nodes[*t].col = 'g';
                    self.nodes[*t].pre = Some(n);
                    queue.push(*t);
                }
            }
            self.nodes[n].col = 'b';
        }

        res
    }
}

#[cfg(test)]
mod bfs_tests {
    use super::AdjacencyList;

    #[test]
    fn bfs_test1() {
        let edges = vec![(0, 1), (0, 4), (1, 5), (5, 2), (5, 6), (2, 6), (2, 3), (6, 3), (6, 7), (7, 3)];
        let mut adjacenct_list = AdjacencyList::initialize(8, edges);
        let paths = adjacenct_list.bfs(1);
        for path in paths {
            println!("{path}");
        }
    }
}
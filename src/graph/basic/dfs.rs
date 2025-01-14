// 邻接链表
pub struct CharAdjacencyList {
    // 字符范围[s, e]
    pub s: char,
    pub e: char,
    // 结点
    pub nodes: Vec<Node>,
    // 邻接链表
    pub ad_list: Vec<Vec<usize>>,
}

#[derive(Debug)]
pub struct Node {
    // 当前值
    pub val: char,
    // 发现时间
    pub d: usize,
    // 完成时间
    pub f: usize,
    // 前驱结点值
    pub pi: Option<char>,
    // 结点颜色, w-未访问, g-已发现, b-该节点的邻接结点都已完成访问
    pub col: char,
}

// 创建邻接链表
pub fn make_adjacency_list(s: char, e: char, mut edges: Vec<(char, char)>) -> CharAdjacencyList {
    let nodes: Vec<Node> = (s..=e)
        .into_iter()
        .map(|c| Node {
            val: c,
            d: 0,
            f: 0,
            pi: None,
            col: 'w',
        })
        .collect();

    let mut adjacency_list: Vec<_> = (s..=e).into_iter().map(|_| Vec::new()).collect();
    let s_asc = s as usize;

    edges.sort_by(|a, b| match a.0.cmp(&b.0) {
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Equal => a.1.cmp(&b.1),
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
    });

    for (x, y) in edges {
        adjacency_list[x as usize - s_asc].push(y as usize - s_asc);
    }

    CharAdjacencyList {
        s,
        e,
        nodes,
        ad_list: adjacency_list,
    }
}

pub fn dfs(list: &mut CharAdjacencyList) {
    let mut times = 0;
    let s_asc = list.s as usize;
    for c in list.s..=list.e {
        let idx = c as usize - s_asc;
        if list.nodes[idx].col == 'w' {
            dfs_visit(&mut list.nodes, &list.ad_list, list.s, c, &mut times);
        }
    }
}

fn dfs_visit(
    nodes: &mut Vec<Node>,
    ad_list: &Vec<Vec<usize>>,
    s: char,
    cur: char,
    times: &mut usize,
) {
    let cur_idx = cur as usize - s as usize;
    *times += 1;
    nodes[cur_idx].col = 'g';
    nodes[cur_idx].d = *times;
    for i in &ad_list[cur_idx] {
        // let a = list.nodes[*i];
        // list.nodes[*i].borrow_mut().col = 'g';
        if nodes[*i].col == 'w' {
            nodes[*i].pi = Some(cur);
            let cur = (s as u8 + *i as u8) as char;
            dfs_visit(nodes, ad_list, s, cur, times);
        }
    }
    *times += 1;
    nodes[cur_idx].f = *times;
    nodes[cur_idx].col = 'b';
}

// 基于栈的dfs
pub fn stack_based_dfs(list: &mut CharAdjacencyList) -> Vec<char> {
    let mut stack = Vec::new();
    let mut times = 0;
    let mut topo_list = Vec::new();

    let s_u = list.s as usize;
    for c in list.s..=list.e {
        let idx = c as usize - s_u;
        if list.nodes[idx].col == 'w' {
            times += 1;
            list.nodes[idx].col = 'g';
            list.nodes[idx].d = times;
            stack.push(idx);
        }

        while !stack.is_empty() {
            let cur_idx = *stack.last().unwrap() as usize;
            let tmp_arr = &list.ad_list[cur_idx];
            let mut next = None;
            for i in tmp_arr {
                if list.nodes[*i].col == 'w' {
                    next = Some(*i);
                    break;
                }
            }
            match next {
                Some(next_idx) => {
                    times += 1;
                    list.nodes[next_idx].col = 'g';
                    list.nodes[next_idx].d = times;
                    list.nodes[next_idx].pi = Some((s_u + cur_idx) as u8 as char);
                    stack.push(next_idx);
                }
                None => {
                    times += 1;
                    list.nodes[cur_idx].col = 'b';
                    list.nodes[cur_idx].f = times;
                    topo_list.push((s_u + cur_idx) as u8 as char);
                    stack.pop();
                }
            }
        }
    }

    topo_list
}

// 拓扑排序
pub fn topological_sort(s: char, e: char, mut edges: Vec<(char, char)>) -> Vec<char> {
    let mut adjacency_list = make_adjacency_list(s, e, edges);
    stack_based_dfs(&mut adjacency_list)
}

// s -> e的路径数
pub fn path_num(list: &mut CharAdjacencyList, s: char, e: char) -> usize {
    if s < list.s || e > list.e || s > e {
        panic!("超出范围或者s > e, s: {}, e: {}", s, e);
    }
    let mut stack = Vec::new();
    let mut node_path_num = vec![(0, 0); list.e as usize - list.s as usize + 1];
    let start_idx = s as usize - list.s as usize;
    stack.push(start_idx);
    while !stack.is_empty() {
        let last = *stack.last().unwrap();
        if node_path_num[last].0 < list.ad_list[last].len() {
            if list.ad_list[last][node_path_num[last].0] + list.s as usize == e as usize {
                node_path_num[last].1 += 1;
            } else {
                let child = list.ad_list[last][node_path_num[last].0];
                if node_path_num[child].0 < list.ad_list[child].len() {
                    stack.push(list.ad_list[last][node_path_num[last].0]);
                }
            }
            node_path_num[last].0 += 1;
        } else {
            for i in &list.ad_list[last] {
                node_path_num[last].1 += node_path_num[*i].1;
            }
            println!("last: {last}, paths: {}, stack: {:?}", node_path_num[last].1, stack);
            stack.pop();
        }
    }
    println!("node_path_num: {:?}", node_path_num);
    node_path_num[s as usize - list.s as usize].1
}

#[cfg(test)]
mod dfs_tests {
    use crate::graph::basic::dfs::{path_num, stack_based_dfs};

    use super::{dfs, make_adjacency_list};

    #[test]
    fn dfs_test1() {
        let mut edges = Vec::new();
        edges.push(('u', 'v'));
        edges.push(('u', 'x'));
        edges.push(('v', 'y'));
        edges.push(('w', 'y'));
        edges.push(('w', 'z'));
        edges.push(('x', 'v'));
        edges.push(('y', 'x'));
        edges.push(('z', 'z'));
        let edges_copy = edges.clone();

        let mut adjacency_list = make_adjacency_list('u', 'z', edges);
        dfs(&mut adjacency_list);
        println!("recursive: {:?}", adjacency_list.nodes);

        let mut adjacency_list_copy = make_adjacency_list('u', 'z', edges_copy);
        println!("{:?}", adjacency_list_copy.ad_list);
        stack_based_dfs(&mut adjacency_list_copy);
        println!("stack: {:?}", adjacency_list_copy.nodes);
    }

    #[test]
    fn topological_sort_test1() {
        let mut edges = Vec::new();
        edges.push(('m', 'q'));
        edges.push(('m', 'r'));
        edges.push(('m', 'x'));
        edges.push(('n', 'q'));
        edges.push(('n', 'u'));
        edges.push(('n', 'o'));
        edges.push(('o', 'r'));
        edges.push(('o', 's'));
        edges.push(('o', 'v'));
        edges.push(('p', 'o'));
        edges.push(('p', 's'));
        edges.push(('p', 'z'));
        edges.push(('q', 't'));
        edges.push(('r', 'u'));
        edges.push(('r', 'y'));
        edges.push(('s', 'r'));
        edges.push(('u', 't'));
        edges.push(('v', 'x'));
        edges.push(('v', 'w'));
        edges.push(('w', 'z'));
        edges.push(('y', 'v'));
        let mut adjacency_list = make_adjacency_list('m', 'z', edges);
        let seq = stack_based_dfs(&mut adjacency_list);
        println!("seq: {:?}", seq);

        let num = path_num(&mut adjacency_list, 'p', 'v');
        assert_eq!(4, num);

        let num = path_num(&mut adjacency_list, 'p', 'z');
        assert_eq!(5, num);
    }
}

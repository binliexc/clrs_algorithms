// 强连通分量

use super::dfs::CharAdjacencyList;

pub fn scc(al: &mut CharAdjacencyList) -> Vec<Vec<char>> {
    let mut orders = dfs_order(al);
    orders.reverse(); 
    let mut ad_list: Vec<_> = (al.s..=al.e).into_iter().map(|_| Vec::new()).collect();
    for i in 0..al.ad_list.len() {
        for n in &al.ad_list[i] {
            ad_list[*n].push(i);
        }
    }
    let len = al.e as usize - al.s as usize + 1;
    let mut visited = vec![('w', 0, 0); len];
    let mut stack = Vec::new();
    let mut time = 0;
    let mut res = Vec::new();
    for c in orders {
        if visited[c].0 == 'w' {
            time += 1;
            visited[c] = ('g', time, 0);
            stack.push(c);
        }

        let mut order = Vec::new();
        while !stack.is_empty() {
            let last = *stack.last().unwrap();
            let mut next_idx = None;
            for i in &ad_list[last] {
                if visited[*i].0 == 'w' {
                    next_idx = Some(*i);
                    break;
                }
            }

            time += 1;
            match next_idx {
                Some(i) => {
                    visited[i] = ('g', time, 0);
                    stack.push(i);
                }
                None => {
                    visited[last].0 = 'b';
                    visited[last].2 = time;
                    order.push((al.s as u8 + last as u8) as char);
                    stack.pop();
                }
            }
        }
        if !order.is_empty() {
            res.push(order);
        }
    } 
    res
}

pub fn dfs_order(al: &mut CharAdjacencyList) -> Vec<usize> {
    let len = al.e as usize - al.s as usize + 1;
    let mut order = Vec::with_capacity(len);
    let mut visited = vec![('w', 0, 0); len];

    let mut stack = Vec::new();
    let mut time = 0;
    for c in al.s..=al.e {
        let cur_idx = c as usize - al.s as usize;
        if visited[cur_idx].0 == 'w' {
            time += 1;
            visited[cur_idx] = ('g', time, 0);
            stack.push(cur_idx);
        }
        while !stack.is_empty() {
            let last = *stack.last().unwrap();
            let mut next_idx = None;
            for i in &al.ad_list[last] {
                if visited[*i].0 == 'w' {
                    next_idx = Some(*i);
                    break;
                }
            }

            time += 1;
            match next_idx {
                Some(i) => {
                    visited[i] = ('g', time, 0);
                    stack.push(i);
                }
                None => {
                    visited[last].0 = 'b';
                    visited[last].2 = time;
                    order.push(last);
                    stack.pop();
                }
            }
        }
    }
    order
}

#[cfg(test)]
mod scc_tests {
    use crate::graph::basic::{dfs::make_adjacency_list, scc::scc};

    use super::dfs_order;

    #[test]
    fn dfs_order_test1() {
        let mut edges = Vec::new();
        edges.push(('a', 'b'));
        edges.push(('b', 'c'));
        edges.push(('b', 'e'));
        edges.push(('b', 'f'));
        edges.push(('c', 'd'));
        edges.push(('c', 'g'));
        edges.push(('d', 'c'));
        edges.push(('d', 'h'));
        edges.push(('e', 'a'));
        edges.push(('e', 'f'));
        edges.push(('f', 'g'));
        edges.push(('g', 'f'));
        edges.push(('g', 'h'));
        edges.push(('h', 'h'));
        let mut al = make_adjacency_list('a', 'h', edges);
        let order = dfs_order(&mut al);
        let order: Vec<char> = dfs_order(&mut al).iter().map(|u| (al.s as u8 + *u as u8) as char).collect();
        println!("order: {:?}", order);
    }

    #[test]
    fn scc_test1() {
        let mut edges = Vec::new();
        edges.push(('a', 'b'));
        edges.push(('b', 'c'));
        edges.push(('b', 'e'));
        edges.push(('b', 'f'));
        edges.push(('c', 'd'));
        edges.push(('c', 'g'));
        edges.push(('d', 'c'));
        edges.push(('d', 'h'));
        edges.push(('e', 'a'));
        edges.push(('e', 'f'));
        edges.push(('f', 'g'));
        edges.push(('g', 'f'));
        edges.push(('g', 'h'));
        edges.push(('h', 'h'));
        let mut al = make_adjacency_list('a', 'h', edges);
        let sccs =  scc(&mut al);
        println!("sccs: {:?}", sccs);
    }

}

// 邻接矩阵表示图G(V, E)

// 实现在O(n)时间复杂度下求通用汇点(入度为|V| - 1, 出度为0)
pub fn universal_sink(matrix: &Vec<Vec<usize>>) -> Option<usize> {
    let mut i = 1;
    let mut j = 1;
    while i < matrix.len() && j < matrix[0].len() {
        if matrix[i][j] == 0 {
            j += 1;
        } else {
            i += 1;
        }
    }
    for val in &matrix[i] {
        if *val == 1 {
            return None;
        }
    }
    return Some(i);
}

// 关联矩阵B, 关联矩阵Bt, 以及BBt的每一项的含义
// 结论: BBt(i, j), 如果i==j, BBt(i, j)等于点i的 出度+入度;
// 否则, BBt(i, j)等于 -(点i和点j的连接数)
pub fn incidence_matrix(range: usize, mut edges: Vec<(usize, usize)>) {
    edges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut incidence_matix = vec![vec![0; edges.len()]; range];
    for idx in 0..edges.len() {
        let (x, y) = edges[idx];
        incidence_matix[x][idx] = -1;
        incidence_matix[y][idx] = 1;
    }

    let mut bbt_matrix = vec![vec![0; range]; range];
    for a in 0..incidence_matix.len() {
        for b in 0..incidence_matix.len() {
            for m in 0..edges.len() {
                bbt_matrix[a][b] += incidence_matix[a][m] * incidence_matix[b][m];
            }
        }
    }

    for i in 0..bbt_matrix.len() {
        for j in 0..bbt_matrix[i].len() {
            print!("{} ", bbt_matrix[i][j]);
        }
        println!("");
    }
}

#[cfg(test)]
mod adjacency_matrix_tests {
    use super::{incidence_matrix, universal_sink};

    #[test]
    fn universal_sink_test1() {
        let a1 = vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 1, 1],
            vec![0, 0, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1],
        ];
        assert_eq!(None, universal_sink(&a1));

        let a2 = vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0],
        ];
        assert_eq!(3, universal_sink(&a2).unwrap());
    }

    #[test]
    fn incidence_matrix_test1() {
        let edges = vec![(0, 3), (1, 3), (1, 4), (2, 3), (3, 4), (3, 5), (4, 5)];
        incidence_matrix(6, edges);
    }
} 
use line_segment_tree::LineSegmentTree;

fn main() {
    let elements = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let tree = LineSegmentTree::new(elements);
    println!("{}", tree.query(5, 8)) 
}
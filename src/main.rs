mod line_segment_tree;
mod binary_indexed_tree;
mod basic_difference;
mod interval_sum;
mod lists;

fn main() {
    // let elements = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let tree = LineSegmentTree::new(elements);
    // println!("{}", tree.query(5, 8)) 

    let mut data = 10;
    let ref1 = &mut data;
    let ref2 = &mut *ref1;
    *ref2 += 1;
    *ref1 += 1;
    println!("{}", data);
}
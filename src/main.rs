#![feature(once_cell_get_mut)]
#![feature(box_as_ptr)]

use disjoint_set::depth_determination::{Container, TreeNode};

mod basic_difference;
mod binary_indexed_tree;
mod disjoint_set;
mod interval_sum;
mod line_segment_tree;
mod lists;

fn main() {
    // let elements = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let tree = LineSegmentTree::new(elements);
    // println!("{}", tree.query(5, 8))

    // let mut data = 10;
    // let ref1 = &mut data;
    // let ref2 = &mut *ref1;
    // *ref2 += 1;
    // *ref1 += 1;
    // println!("{}", data);

    // disjoint_set_test1();

    depth_determination_test1();
}

pub fn depth_determination_test1() {
    let mut container = Container::new(14);
    for i in 0..14 {
        container.make_tree(i);
    }
    
    // Tree1
    container.graft(5, 4);
    container.graft(4, 2);
    container.graft(2, 1);

    // Tree2
    container.graft(13, 12);
    container.graft(12, 3);

    // Tree3
    container.graft(10, 8);
    container.graft(9, 8);
    container.graft(8, 6);
    container.graft(7, 6);

    container.graft(3, 1);
    container.graft(6, 3);

    println!("node10.depth: {}", container.find_depth(10));
    println!("node7.depth: {}", container.find_depth(7));
    println!("node12.depth: {}", container.find_depth(12));
    println!("node13.depth: {}", container.find_depth(13));
    println!("node1.depth: {}", container.find_depth(1));
    println!("node5.depth: {}", container.find_depth(5));
}

#![feature(once_cell_get_mut)]
use std::mem::ManuallyDrop;

use disjoint_set::{Set, NODE_CONTAINER};

mod basic_difference;
mod binary_indexed_tree;
mod interval_sum;
mod line_segment_tree;
mod lists;
mod disjoint_set;

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

    disjoint_set_test1();
}

fn disjoint_set_test1() {
    let nodes = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let edges = vec![(1, 4), (5, 7), (1, 3), (8, 9), (1, 2), (5, 6), (2, 3)];

    for node in nodes {
        Set::make_set(node);
    }

    for (a, b) in edges {
        let set_a = Set::find_set(a);
        let set_b = Set::find_set(b);
        println!("a: {a}, b: {b}");
        println!("set_a: {:?}", set_a);
        println!("set_b: {:?}", set_b);

        Set::union(set_a, set_b);
    }
    println!("union set complete");

    let container = unsafe { NODE_CONTAINER.get().unwrap() };
    for (key, val) in container {
        let n = unsafe { Box::from_raw(*val) };
        println!("key: {key}, set: {:p}", n.set);
        let _ = ManuallyDrop::new(n);
    }
}

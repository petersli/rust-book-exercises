//! In this file, you will implement two simple algorithms.
//! The goal is to familiarize you with the basics of working with references.
//!
//! Both of these problems involve the `Vec` datatype. I would take a look the `Vec` documentation:
//! https://doc.rust-lang.org/std/vec/struct.Vec.html

/// P1a: `insort` is a function that takes a sorted vector `v`, and inserts an element `n` into `v`
/// such that `v` remains sorted.
///
/// You may assume that `v` is already sorted, and do not need to check this fact.
///
/// Run `cargo test insort` to check your answers.
pub fn insort(v: &mut Vec<i32>, n: i32) {
    let mut insert_at = v.len();
    for (index, num) in v.iter().enumerate() {
        if n < *num {
            insert_at = index;
            break;
        }
    }
    v.insert(insert_at, n)
}

type Node = i32;

/// P1b: `connected` is a function that takes an edge-list representation `edges` of a *directed* graph
/// (i.e. edges has the form `[(&from, &to), ...]`) as well as a source `src` and destination `dst`.
/// `connected` returns true if there exists a path from `src` to `dst` in `edges`.
///
/// Note: in this graph representation, references to nodes are not e.g. indices into a vector, but actual
/// Rust references. You need to be very careful when comparing two nodes for equality. For example, in the program:
///   
///    let x = 1; let y = 1;
///    assert!(&x == &y)
///
/// Then this assertion passes because Rust does an implicit dereference on equality checks. You will need
/// to use the [`std::ptr::eq`](https://doc.rust-lang.org/std/ptr/fn.eq.html) function to implement `connected`.
///
/// Run `cargo test connected` to check your answers.
use std::collections::{HashSet, VecDeque};
pub fn connected(edges: &[(&Node, &Node)], src: &Node, dst: &Node) -> bool {
    let mut seen = HashSet::from([src as *const i32]);
    let mut queue = VecDeque::from([src as *const i32]);
    while !queue.is_empty() {
        let current_node = queue.pop_front().unwrap();
        if std::ptr::eq(current_node, dst) {
            return true;
        }
        for &(start_node, end_node) in edges {
            if std::ptr::eq(start_node, current_node) && !seen.contains(&(end_node as *const i32)) {
                seen.insert(end_node as *const i32);
                queue.push_back(end_node as *const i32)
            }
        }
    }
    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insort_test() {
        let mut v = vec![1, 5, 8];

        insort(&mut v, 0);
        assert_eq!(v, vec![0, 1, 5, 8]);

        insort(&mut v, 3);
        assert_eq!(v, vec![0, 1, 3, 5, 8]);

        insort(&mut v, 9);
        assert_eq!(v, vec![0, 1, 3, 5, 8, 9]);
    }

    #[test]
    fn connected_test() {
        let nodes = vec![1, 1, 1];
        let edges = vec![(&nodes[0], &nodes[1]), (&nodes[1], &nodes[2])];
        assert!(connected(&edges, &nodes[0], &nodes[2]));
        assert!(!connected(&edges, &nodes[2], &nodes[0]))
    }
}

use std::cmp::max;
use std::cmp::min;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/*
* The following function takes as input the path of the input file.
* It returns two values:
*  - values: the array A[1,n] on which the operations will be performed;
*  - queries: the array that contains the queries which can be a triple like
*   <i, j, T> or a pair like <i, j>. The queries are distinguished by a prefix
*   which can be 0 or 1. The prefix 0 corresponds to the query Update and the prefix
*   1 corresponds to the query Max.
*/
pub fn read_data(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut values: Vec<i32> = Vec::new();
    let file = BufReader::new(File::open(file_path).unwrap());

    for line in file.lines() {
        let numbers: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().expect("Error during parsing"))
            .collect();

        for num in numbers {
            values.push(num);
        }
    }

    let queries = values.split_off((values[0] + 2) as usize);

    values.drain(0..2);

    let n = values.len();
    let mut diff = n.next_power_of_two() - n;

    while diff > 0 {
        values.push(-1);
        diff -= 1;
    }

    (values, queries)
}

/*
 * The following function builds the segment tree given the array A[1,n] of values. In this case
 * the nodes of the tree contain the max between the left and right child.
 */
pub fn build_segment_tree(arr: &mut Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    let mut segment_tree = vec![0; 2 * n - 1];

    segment_tree[n - 1..2 * n - 1].copy_from_slice(arr);

    for i in (0..n - 1).rev() {
        segment_tree[i] = max(segment_tree[2 * i + 1], segment_tree[2 * i + 2]);
    }

    segment_tree
}

/*
 * The following function it is a recursive one and it computes the update for a given range and a given value
 *
 * The input parameters are:
 * - segment_tree: it is the segment tree
 * - lazy_tree: it is the lazy tree used for the lazy propagation
 * - node: it is the index in the segment tree corresponding to the current node
 * - start: it is the startign range represented by the current node
 * - end: it is the ending range represented by the current node
 * - i: it is the starting range of the query
 * - j: it is the ending range of the query
 * - val: it is the value that must be compared with A[k] where i <= k <= j
 */
pub fn update_range_rec(
    segment_tree: &mut Vec<i32>,
    lazy_tree: &mut Vec<i32>,
    node: usize,
    start: usize,
    end: usize,
    i: usize,
    j: usize,
    val: i32,
) {
    if lazy_tree[node] != 0 {
        segment_tree[node] = min(lazy_tree[node], segment_tree[node]);
        if start != end {
            lazy_tree[node * 2 + 1] = min(lazy_tree[node], segment_tree[node * 2 + 1]);
            lazy_tree[node * 2 + 2] = min(lazy_tree[node], segment_tree[node * 2 + 2]);
        }
        lazy_tree[node] = 0;
    }

    if start > end || start > j || end < i {
        return;
    }

    if start >= i && end <= j {
        let tmp = min(segment_tree[node], val);
        segment_tree[node] = tmp;

        if start != end {
            if tmp < segment_tree[node * 2 + 1] {
                lazy_tree[node * 2 + 1] = tmp;
            }

            if tmp < segment_tree[node * 2 + 2] {
                lazy_tree[node * 2 + 2] = tmp;
            }
        }
        return;
    }

    let mid = start + (end - start) / 2;

    update_range_rec(segment_tree, lazy_tree, node * 2 + 1, start, mid, i, j, val);
    update_range_rec(
        segment_tree,
        lazy_tree,
        node * 2 + 2,
        mid + 1,
        end,
        i,
        j,
        val,
    );
    segment_tree[node] = max(segment_tree[node * 2 + 1], segment_tree[node * 2 + 2]);
}

/*
 * The following function computes the update calling the recursive function.
 * The input parameters are:
 * - segment_tree: it is the segment tree;
 * - lazy_tree: it is the lazy tree;
 * - i: it is the starting index of the query to apply
 * - j: it is the ending index of the query to apply
 * - val: it is the value to be compared
 */
pub fn update_range(
    segment_tree: &mut Vec<i32>,
    lazy_tree: &mut Vec<i32>,
    i: usize,
    j: usize,
    val: i32,
) {
    let n = (segment_tree.len() - 1) / 2;
    update_range_rec(segment_tree, lazy_tree, 0, 0, n, i, j, val);
}

/*
 * The following function it is a recursive one and it computes the max for a given range
 *
 * The input parameters are:
 * - segment_tree: it is the segment tree
 * - lazy_tree: it is the lazy tree used for the lazy propagation
 * - node: it is the index in the segment tree corresponding to the current node
 * - start: it is the startign range represented by the current node
 * - end: it is the ending range represented by the current node
 * - i: it is the starting range of the query
 * - j: it is the ending range of the query
 */
pub fn ranged_max_rec(
    segment_tree: &mut Vec<i32>,
    lazy_tree: &mut Vec<i32>,
    node: usize,
    start: usize,
    end: usize,
    i: &mut usize,
    j: &mut usize,
) -> i32 {
    if lazy_tree[node] != 0 {
        segment_tree[node] = lazy_tree[node];

        if start != end {
            lazy_tree[node * 2 + 1] = min(lazy_tree[node], segment_tree[node * 2 + 1]);
            lazy_tree[node * 2 + 2] = min(lazy_tree[node], segment_tree[node * 2 + 2]);
        }

        lazy_tree[node] = 0;
    }

    if start > end || start > *j || end < *i {
        return -1000;
    }
    if start >= *i && end <= *j {
        return segment_tree[node];
    }

    let mid = start + (end - start) / 2;
    let p1 = ranged_max_rec(segment_tree, lazy_tree, node * 2 + 1, start, mid, i, j);
    let p2 = ranged_max_rec(segment_tree, lazy_tree, node * 2 + 2, mid + 1, end, i, j);

    max(p1, p2)
}

/*
 * The following function computes the max calling the recursive function.
 * The input parameters are:
 * - segment_tree: it is the segment tree;
 * - lazy_tree: it is the lazy tree;
 * - i: it is the starting index of the query to apply
 * - j: it is the ending index of the query to apply
 */
pub fn ranged_max(
    segment_tree: &mut Vec<i32>,
    lazy_tree: &mut Vec<i32>,
    i: &mut usize,
    j: &mut usize,
) -> i32 {
    let n = (segment_tree.len() - 1) / 2;

    ranged_max_rec(segment_tree, lazy_tree, 0, 0, n, i, j)
}

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/*
* The following function takes as input the path of the input file.
* It returns three values:
*  - values: the array A[1,n] on which the operations will be performed;
*  - operations: the array of length 3*m that contains all the m operations concatenated,
*       each of them is a triple <l, r, d>;
*  - queries: the array of length 2*k that contains all the k queries concatenated,
*       each of them is a pair <a, b>.
*/
pub fn read_data(file_path: &str) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut values: Vec<i32> = Vec::new();
    let file = BufReader::new(File::open(file_path).unwrap());

    // we read all the numbers and are pushed in the vector values
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

    // we take the size of the array A and the number of the operations
    let n = values[0] as usize;
    let m = values[1] as usize;

    // we split values, queries and operations
    let queries = values.split_off((n + 3 * m + 3) as usize);
    let operations = values.split_off((n + 3) as usize);

    // we drop the first three elements which are the sizes
    values.drain(0..3);

    // we check if the array is a power of two and if it is not we add "dummy" values
    // to reach the power of two (in this case we add the value 0)
    let mut diff = n.next_power_of_two() - n;

    while diff > 0 {
        values.push(0);
        diff -= 1;
    }

    (values, operations, queries)
}

/*
 * The following function builds the segment tree given the array A[1,n] of values. In this case
 * the nodes of the tree contain the sum of the left and right child.
 */
pub fn build_segment_tree(arr: &mut Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    let mut segment_tree = vec![0; 2 * n - 1]; // it has size 2*n - 1

    segment_tree[n - 1..2 * n - 1].copy_from_slice(arr);

    for i in (0..n - 1).rev() {
        segment_tree[i] = segment_tree[2 * i + 1] + segment_tree[2 * i + 2];
    }

    segment_tree
}

/*
 *      START
 * In this section it is used the approach of the difference array in order to compute how many times
 * an operation must be executed such that instead of executing the operation i-th several times, we preprocess
 * a prior the number of times it must be executed and then we perform the operation one time, but the adding value
 * will be multiplied by the number of times the operation must be executed.
 *
 *
 * EXAMPLE: if the operation <2, 5, 3> should be executed 4 times, we execute it one time and instead of adding 3,
 * we add (3*4) = 12.
 */

/*
 * The following function uses the difference array to compute the update of <l, r, 1> in constant time O(1).
 */
pub fn preprocess_one(diff_vector: &mut Vec<i32>, l: usize, r: usize) {
    diff_vector[l] += 1;

    diff_vector[r + 1] -= 1;
}

/*
 * The following function takes as input the vector of queries and m which is the number of operations.
 * It returns counters which is a vector of size m and for each position i-th it contains how many times the
 * operation i-th must be executed. (This function uses the difference array).
 */
pub fn preprocess(queries: Vec<i32>, m: usize) -> Vec<i32> {
    let mut counters = vec![0; m];
    let mut diff_array = vec![0; m + 1]; // difference array

    let mut i = 0;

    // we process all the queries one by one
    while i < queries.len() {
        preprocess_one(
            &mut diff_array,
            (queries[i] - 1) as usize,
            (queries[i + 1] - 1) as usize,
        ); // we subtract 1 because the indexes start from 1 instead of 0
        i += 2;
    }

    // "print_array" function: it computes the value of each operation
    for i in 0..counters.len() {
        if i == 0 {
            counters[i] = diff_array[i];
        } else {
            counters[i] = diff_array[i] + counters[i - 1];
        }
    }

    counters
}

/*
 * At this point we have computed for each operation in operations[1, m], how many times it must be executed.
 *
 *      END difference array section
 */

/*
 * The following function is recursive and it computes the update of a generic operation <l , r, d*counter>.
 * It uses the lazy propagation in order to get a better time complexity.
 * The input parameters are:
 * - segment_tree: it is the segment tree
 * - lazy_tree: it is the lazy tree used in the lazy propagation
 * - qlow: it is the starting index on which the query must be applied
 * - qhigh: it is the ending index on which the query must be applied
 * - low: it is the starting index of the array A represented by the current node
 * - high: it is the ending index  of the array Arepresented by the current node
 * - pos: it is index of the current node in the segment tree
 * - val: it is the value to be added
 * - counter: it is the number of times val must be added
 */

pub fn update_rec(
    segment_tree: &mut Vec<i32>,
    lazy_tree: &mut Vec<i32>,
    qlow: usize,
    qhigh: usize,
    low: usize,
    high: usize,
    pos: usize,
    val: i32,
    counter: i32,
) {
    if low > high {
        return;
    }

    // case in which we wrote on the lazy tree
    if lazy_tree[pos] != 0 {
        segment_tree[pos] += lazy_tree[pos];
        if low != high {
            lazy_tree[2 * pos + 1] += lazy_tree[pos];
            lazy_tree[2 * pos + 2] += lazy_tree[pos];
        }
        lazy_tree[pos] = 0;
    }

    if qlow > high || qhigh < low {
        return;
    }

    if qlow <= low && qhigh >= high {
        segment_tree[pos] += val * counter;
        if low != high {
            lazy_tree[2 * pos + 1] += val * counter;
            lazy_tree[2 * pos + 2] += val * counter;
        }
        return;
    }

    let mid = (low + high) / 2;
    update_rec(
        segment_tree,
        lazy_tree,
        qlow,
        qhigh,
        low,
        mid,
        2 * pos + 1,
        val,
        counter,
    );
    update_rec(
        segment_tree,
        lazy_tree,
        qlow,
        qhigh,
        mid + 1,
        high,
        2 * pos + 2,
        val,
        counter,
    );
    segment_tree[pos] = segment_tree[2 * pos + 1] + segment_tree[2 * pos + 2];
}

/*
 * It is the function that computes the update. Its input parameters are:
 * - segment_tree: it is the segment tree
 * - lazy_tree: it is the lazy tree used for the lazy propagation
 * - n: it is the length of the vector values (i.e. it is the length of the array A filled with 0s if necessary)
 * - operations: it is the vector that contains all the triples <l, r, d>.
 * - counters: it is the vector containing for each operation the number of times it must be executed
 */
pub fn update(
    segment_tree: &mut Vec<i32>,
    lazy_tree: &mut Vec<i32>,
    n: usize,
    operations: Vec<i32>,
    counters: Vec<i32>,
) {
    let mut i = 0;
    let mut j = 0;

    // it is performed the update function recursively for each operation one by one
    while i < operations.len() {
        update_rec(
            segment_tree,
            lazy_tree,
            (operations[i] - 1) as usize,
            (operations[i + 1] - 1) as usize,
            0,
            n - 1,
            0,
            operations[i + 2],
            counters[j],
        );
        j += 1; // it is used to move on the counter of the next operation
        i += 3; // it is used to move on the next operation (recall the fact that the operations are concatenated)
    }
}

/*
 * The following function returns the result for a generic index.
 * It takes as input parameter:
 * - segment_tree: it is the segment tree
 * - lazy_tree: it is the lazy tree
 * - q: it is the index on which we want to compute the value
 * - low: it is the starting index of the array A represented by the current node
 * - high: it is the ending index  of the array Arepresented by the current node
 * - pos: it is index of the current node in the segment tree
 */
pub fn get_result(
    segment_tree: &mut Vec<i32>,
    lazy_tree: &mut Vec<i32>,
    q: usize,
    low: usize,
    high: usize,
    pos: usize,
) -> i32 {
    if low > high {
        return 0;
    }

    if lazy_tree[pos] != 0 {
        segment_tree[pos] += lazy_tree[pos];
        if low != high {
            lazy_tree[2 * pos + 1] += lazy_tree[pos];
            lazy_tree[2 * pos + 2] += lazy_tree[pos];
        }
        lazy_tree[pos] = 0;
    }

    if q > high || q < low {
        return 0;
    }

    if q <= low && q >= high {
        return segment_tree[pos];
    }

    let mid = (low + high) / 2;
    get_result(segment_tree, lazy_tree, q, low, mid, 2 * pos + 1)
        + get_result(segment_tree, lazy_tree, q, mid + 1, high, 2 * pos + 2)
}

/*
 * The following function computes the results for the whole array and it returns the vector
 * containing the solutions.
 * It takes as input:
 * - segment_tree: it is the segment tree
 * - lazy_tree: it is the lazy tree
 * - n: it is the length of the vector value (i.e the array A), 
    but notice that in this case it contains even the "dummy" values (0) used for filling in order to reach the power of two.
 */
pub fn get_results(segment_tree: &mut Vec<i32>, lazy_tree: &mut Vec<i32>, n: usize) -> Vec<i32> {
    let mut results = Vec::new();

    for i in 0..n {
        let result = get_result(segment_tree, lazy_tree, i, 0, n - 1, 0);
        if result == 0 {
            break;
        }
        results.push(result);
    }

    results
}

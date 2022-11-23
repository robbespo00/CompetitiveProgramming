use std::env;
use queries_and_operations::{
    build_segment_tree, get_results, preprocess, read_data, update,
};


fn main() {

    // read the input argument
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let file_path = format!("Testset/{}", file);

    // read the data from the input file
    let mut values;
    let operations;
    let queries;
    (values, operations, queries) = read_data(&file_path);
    let n = values.len();

    // build segment tree
    let mut segment_tree = build_segment_tree(&mut values);
    // intialize the lazy tree
    let mut lazy_tree = vec![0; segment_tree.len()];

    // computes how many times a operation must be executed
    let counters = preprocess(queries, operations.len() / 3);

    // executes the operations
    update(&mut segment_tree, &mut lazy_tree, n, operations, counters);

    // computes the results
    let solution = get_results(&mut segment_tree, &mut lazy_tree, n);

    // print the results (it is used this form in order to avoid the [] brackets)
    solution.iter().for_each(|val| print!("{} ", val));
    println!();
}

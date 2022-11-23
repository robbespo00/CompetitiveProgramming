use std::env;
use min_and_max::{build_segment_tree, ranged_max, read_data, update_range};

fn main() {

    // read the input argument
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let file_path = format!("Testset/{}", file);


    // read the data from the input file
    let mut values;
    let queries;
    (values, queries) = read_data(&file_path);

    // build the segment tree
    let mut segment_tree = build_segment_tree(&mut values);
    // initialize the lazy tree
    let mut lazy_tree = vec![0; segment_tree.len()];

    let mut rng_max;

    let mut i;
    let mut j;
    let mut k = 0;

    while k < queries.len() {
        if queries[k] == 0 {
            update_range(
                &mut segment_tree,
                &mut lazy_tree,
                (queries[k + 1] - 1) as usize, // because the indexes start from 1 not 0
                (queries[k + 2] - 1) as usize, 
                queries[k + 3],
            );
            k += 4;
        } else {
            i = (queries[k + 1] - 1) as usize; // because the indexes start from 1 not 0
            j = (queries[k + 2] - 1) as usize;

            rng_max = ranged_max(&mut segment_tree, &mut lazy_tree, &mut i, &mut j);
            println!("{:?}", rng_max);
            k += 3;
        }
    }
}

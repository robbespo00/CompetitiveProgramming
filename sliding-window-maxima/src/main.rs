mod solution_deque;
mod solution_trivial;
use solution_deque::solution_deque;
use solution_trivial::solution_trivial;

fn main() {
    let nums = vec![1, 3, 2, 1, 5, 3, 2, 6];
    let k = 3;

    // let deq_ans = solution_deque(nums, k);
    // println!("Deque solution: {:?}", deq_ans);

    let triv_ans = solution_trivial(nums, k);
    println!("Trivial solution: {:?}", triv_ans);
}

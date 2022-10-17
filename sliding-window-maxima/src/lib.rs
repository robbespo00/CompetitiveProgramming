use binary_search_tree::BinarySearchTree;
use rand::Rng;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

pub fn brute_force(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = v.len();
    let mut maximums = Vec::with_capacity(n - k + 1);
    for i in 0..(n - k + 1) {
        let current_slice = &v[i..i + k];
        let max_value = *current_slice.iter().max().unwrap();
        maximums.push(max_value);
    }
    maximums
}

pub fn brute_force_idiomatic(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    v.windows(k).map(|w| *w.iter().max().unwrap()).collect()
}

pub fn heap(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    todo!();
}

pub fn bst(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    todo!();
}

pub fn linear(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let mut deq = VecDeque::new(); // it stores the positions, not the values!
    let mut answer = Vec::new(); // it stores the maxima.

    for i in 0..nums.len() {
        while (!deq.is_empty()) && (*deq.front().unwrap() as i32) <= ((i as i32) - k) {
            deq.pop_front();
        }
        while (!deq.is_empty()) && nums[i] >= nums[*deq.back().unwrap()] {
            deq.pop_back();
        }
        deq.push_back(i);
        if (i as i32) >= (k - 1) {
            answer.push(nums[*deq.front().unwrap()]);
        }
    }
    return answer;
}

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        nums.push(rng.gen_range(0..i32::MAX));
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_idiomatic_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = brute_force_idiomatic(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[ignore]
    #[test]
    fn test_heap_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[ignore]
    #[test]
    fn test_bst_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[ignore]
    #[test]
    fn test_linear_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }
}
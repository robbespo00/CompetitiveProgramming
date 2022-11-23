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
    let k = k as usize; // it avoids errors about casting
    let n = nums.len();
    let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    let mut answer = Vec::with_capacity(n - k + 1); // it stores the results.

    for (i, item) in nums.iter().enumerate().take(n) {
        heap.push((*item, i));

        // it saves the maximum element iff it belongs to the window
        if i + 1 >= k {
            answer.push(heap.peek().unwrap().0);
        }

        // it removes all the maxima elements which do not belong to the window anymore
        while !heap.is_empty() && i - heap.peek().unwrap().1 + 1 >= k {
            heap.pop();
        }
    }

    answer
}

pub fn bst(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize; // it avoids errors about casting
    let n = nums.len();
    let mut bst = BinarySearchTree::new();
    let mut answer = Vec::with_capacity(n - k + 1); // it stores the results.

    // it insert in the binary search tree the first k elements of the input
    for item in nums.iter().take(k) {
        bst.insert(*item);
    }

    // at each iteration it saves the result, remove the element out of the window and insert the new element
    for i in k..n {
        answer.push(*bst.max().unwrap());

        bst.remove(&nums[i - k]);
        bst.insert(nums[i]);
    }

    answer.push(*bst.max().unwrap());

    answer
}

pub fn linear(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let mut deq = VecDeque::new(); // it stores the positions, not the values!
    let n = nums.len();
    let mut answer = Vec::new(); // it stores the results.

    for i in 0..n {
        // it removes the elements that do not belong to the window
        while (!deq.is_empty()) && (*deq.front().unwrap() as i32) <= ((i as i32) - k) {
            deq.pop_front();
        }

        // it removes the element smaller than the new element to insert in the window
        while (!deq.is_empty()) && nums[i] >= nums[*deq.back().unwrap()] {
            deq.pop_back();
        }

        deq.push_back(i);

        if i as i32 >= k - 1 {
            answer.push(nums[*deq.front().unwrap()]);
        }
    }

    answer
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

    #[test]
    fn test_idiomatic_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = brute_force_idiomatic(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_heap_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_bst_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_linear_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }
}

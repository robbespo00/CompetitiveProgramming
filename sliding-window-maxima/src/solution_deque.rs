use std::collections::VecDeque;

pub fn solution_deque(nums: Vec<i32>, k: i32) -> Vec<i32> {
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

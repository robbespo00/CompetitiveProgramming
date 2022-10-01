pub fn solution_optimal(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut sum = nums[0];
        
        for i in 1..nums.len(){
            if sum > 0  {sum = sum + nums[i]} else {sum = nums[i]};
            
            if sum > max {
                max = sum;
            }
        }
        
        return max;
        
}
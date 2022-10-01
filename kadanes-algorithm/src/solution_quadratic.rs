pub fn solution_quadratic(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut sum;
    
    for i in 0..nums.len(){
        sum = 0;
        for j in i..nums.len(){
            sum = sum + nums[j];
            if sum > max {
                max = sum;
            }
        }
    }
    return max;
}
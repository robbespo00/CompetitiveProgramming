pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    
    let mut ans = Vec::new();
    for i in 0..nums.len() {
        ans.push(nums[nums[i as usize] as usize]);
    }
    return ans;
}


fn main() {
    let nums = vec![5,0,1,2,3,4];
    let ans = build_array(nums);
    println!("{:?}", ans);
}

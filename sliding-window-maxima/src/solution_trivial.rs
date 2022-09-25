pub fn solution_trivial(nums: Vec<i32>, k: i32) -> Vec<i32> {
        
        let mut answer = Vec::new();
        let mut max;
        
        for i in 0..((nums.len() as i32) -k+1){
            max = nums[i as usize];
            for j in i..(i+k){
                if max < nums[j as usize] {
                    max = nums[j as usize];
                }
            }
            answer.push(max);
        }
        
        return answer;
}
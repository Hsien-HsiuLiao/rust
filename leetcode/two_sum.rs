impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        /*
        iterate through nums vector 
        start with 0 and add first element with second/compare to target, then first with third, etc
        if == target, push into output vec
        */
        let mut output: Vec<i32>= Vec::new();
        for element in 0..nums.len() {
            for i in 1..nums.len() {
                if (nums[element] + nums[i] == target){
                    output.push(element as i32);
                    output.push(i as i32);
                    
                }
            }
        }
        output
    }
}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut original = nums[0];
        
        let mut k:i32 = 1; 
        for index in 1..nums.len() {
            if original != nums[index] {
                original = nums[index];
                nums[k as usize] = nums[index];
                 k += 1;
            }
        }
        
        return k;
        
    }
} 

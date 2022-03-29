impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if(x < 0) {
            return false;
        }
        
        let mut number:Vec<u32> = vec![];
        let mut x:u32 = x as u32;
        while x != 0 {
            number.push(x % 10);
            x = x / 10;
        }
        
        let mut reverse = number.clone();
        reverse.reverse();
        
        number == reverse
    }
} 

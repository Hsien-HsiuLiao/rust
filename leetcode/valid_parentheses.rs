impl Solution {
    pub fn is_valid(s: String) -> bool {
        /*
        
        loop through vecstr until closing found
        if found, check prev and remove if match
        loop again from beginning of vecstr
        else break
        */
        let mut vec_str:Vec<char> = Vec::new();
        let vec_iter = vec_str.iter();
        for c in s.chars() {
            vec_str.push(c);
        }
        let mut i = 1;
       while(i < vec_str.len()){
           if ( vec_str[i] == ')' && vec_str[i-1] == '(' ) 
                || ( vec_str[i] == '}' && vec_str[i-1] == '{' )
                || ( vec_str[i] == ']' && vec_str[i-1] == '[' ) {
               vec_str.remove(i);
               vec_str.remove(i-1);
               i = 1;
           } else {
               i += 1;
           }
        }
        
        println!("{:?}", vec_str);
        vec_str.len() == 0
    }
}

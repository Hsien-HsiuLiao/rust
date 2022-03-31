impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        /*
        compare first char of each string(each element in vector)
        if match for all elements, store char in output vector
        return output vector*/
        let mut common_prefix = String::new();
        let mut n_char = 0;
        let mut common = true;
       
        while(common){
            for index in 1..strs.len() {
                println!("index is {}", index);
                common = (strs[0].chars().nth(n_char) == strs[index].chars().nth(n_char));
                println!("{:?}", strs[index].chars().nth(n_char));
                if(common == false){
                    println!("common is {}", common);
                    break;
                }
                if(index == strs.len()-1){
                    common_prefix.push_str(&strs[0].chars().nth(n_char).unwrap().to_string());
                }
            }
            n_char += 1;
        }
       
    common_prefix
    }
}

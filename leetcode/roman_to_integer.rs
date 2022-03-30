impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut roman: Vec<char> = Vec::new();
        let mut integer:i32 = 0;
        let mut index:usize = 0;
        for c in s.chars() {
            roman.push(c);
        }
    /*    println!("{:?}", roman);
        fn check_subtract(r1: char, r2: char, factor: i32) { 
                            match roman[index + 1] {
                                r1 => {
                                        integer += 4*factor;
                                        index += 1;
                                        },
                                r2 =>  {
                                        integer += 9*factor;
                                        index += 1;
                                         },
                                _ => integer += 1*factor,
                            }                              
        }
      */  
        while (index < roman.len()){
            match roman[index] {
                'I' => { if (index + 1 < roman.len() ){ 
                            match roman[index + 1] {
                                'V' => {
                                        integer += 4;
                                        index += 1;
                                        },
                                'X' =>  {
                                        integer += 9;
                                        index += 1;
                                         },
                                _ => integer += 1,
                            } 
                    } else {
                            integer += 1;
                            }                       
                    },
                'V' => integer += 5,
                'X' => { if (index + 1 < roman.len() ){ 
                            match roman[index + 1] {
                                'L' => {
                                        integer += 40;
                                        index += 1;
                                        },
                                'C' =>  {
                                        integer += 90;
                                        index += 1;
                                         },
                                _ => integer += 10,
                            } 
                    } else {
                            integer += 10;
                            }                       
                    },
                'L' => integer += 50,
                'C' => { if (index + 1 < roman.len() ){ 
                            match roman[index + 1] {
                                'D' => {
                                        integer += 400;
                                        index += 1;
                                        },
                                'M' =>  {
                                        integer += 900;
                                        index += 1;
                                         },
                                _ => integer += 100,
                            } 
                    } else {
                            integer += 100;
                            }                       
                    },
                'D' => integer += 500,
                'M' => integer += 1000,
                _ => ()
            }
            index += 1;
        }
        /*
        check each character and
        add to integer variable if not I, X, C
        
        if I, X, C, check next character
        for I, check for V or X, then add 4 or 9, otherwise add whatever is found
        for X, L C
        for C, D M
        
        return integer
        
        */
        integer
    }
}

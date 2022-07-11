// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_word_count: HashMap<&str, i32> = HashMap::new();
    for word in magazine {
        //create hashmap of words and how many there are
        magazine_word_count.entry(word)
        .and_modify(|copies_of_word| { *copies_of_word += 1})
        .or_insert(1);
    }
    for word in note {
        // check if word to make note exists in magazine
         // and also if there are enough repeat words
        magazine_word_count.entry(word)
        .and_modify(|words_remaining| { *words_remaining -= 1})
        .or_insert(-1);
    }
    //-1 is failing condition, not enough words or doesn't exist
    let mag_has_enough_words = !(magazine_word_count
                                        .values()
                                        .any(|words_remaining| *words_remaining < 0));
    mag_has_enough_words
}

/*
Word Grouping을 이용하여 검색을 해보자
Hashmaps, Nested loop방식을 이용함
*/

use std::collections::Hashmap;

fn word_groupings(words_list: Vec<String>){ Vec<Vec<String>>{
    let mut word_hash = Hashmap::new();

    let mut char_freq = vec![0;26];
    
    //이렇게 하는 이유 : 스펠링이 섞여있는 경우에도 같은 단어로 인식하게 하기 위함...
    for current_word in words_list{
        for c in current_word.to_lowercase().chars(){
            char_freq[(c as u32 - 'a' as u32) as usize] +=1;
        }

        let key = char_freq.into_iter().map(|i|i.to_string()).collect::<String>();
        word_hash.entry(key).or_insert(Vec::new()).push(current_word);
        char_freq = vec![0;26];
    }

    for (key, value) in &word_hash{
        println!("key # {:?} value {:?}", key, value);
    }

    word_hash.into_iter().map(|(_, v)| v).collect()
}

}

fn main(){


    let words = vec!["the".to_string(), "teh".to_string(), "het".to_string(), "apple".to_string(), "aple".to_string(), "apel".to_string()];

    let grouping = word_groupings(words);

    let input_word = String::from("teh");
    for i in grouping.into_iter() {
        if i.contains(&input_word){
            println!("이 단어는 {:?}", i);
        }
    }

}
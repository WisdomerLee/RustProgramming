/*
함수 입력, coercion
 */

use core::num;

fn vowels(words: &String) -> u8{
    let voewls_count = words.chars().into_iter()
    .filter(|x| (*x=='a')||(*x=='e')||(*x=='i')||(*x=='o')||(*x=='u')).count();
    vowels_count as u8
}

//&Box<T> -> &T
fn length_str(x: &Box<&str>){
    println!("{} 문자 길이 {}", x, x.len());
}
//&vec<T> -> &[T]
// fn square_values(num_vec: &Vec<i32>){
//     for i in num_vec{
//         println!("값 {}", i*i);
//     }
// }
fn square_values(num_vec: &[i32]){
    for i in num_vec{
        println!("값 {}", i*i);
    }
}

fn main(){
    let affan = "affan".to_string();
    println!("{}:{:?}", affan, vowels(&affan));
    
    let box_str = Box::new("안녕");
    length_str(&box_str);
    length_str("문자열 직접 입력");

    let values_vec = vec![1,2,3,5,6];
    let values_array = [1,5,6,2,4,6];

    square_values(&values_vec);
    square_values(&values_array);
}
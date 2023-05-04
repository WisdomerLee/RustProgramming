/*
Hash maps

 */
use std::collections::HashMap;
fn main(){
    let mut person:HashMap<&str, i32> = HashMap::new();
    person.insert("아무개", 40);
    person.insert("초소", 55);
    person.insert("진", 80);

    println!("나이 {:?}", person.get("진").unwrap());

    if person.contains_key("아무개"){
        println!("아무개 있음");
    }else{
        println!("아무개 없음");
    }

    match person.get("초소"){
        Some(value) => println!("값이 있음 {}", value),
        None => println!("값이 없음"),
    }

    for (name, age) in &person{
        println!("{}는 {} 살입니다", name, age);
    }

    let mut likes:HashMap<&str, &str> = HashMap::new();

    likes.insert("아무개", "사과");
    //같은 키로 값을 여럿 집어넣으면 가장 마지막에 넣은 값이 살아남는다..
    likes.insert("아무개", "망고");
    println!("과일 좋아하는 것 {:?}", likes);
    likes.entry("아무개").or_insert("사과");//아무개가 키로 있는지 확인하고... 없으면 사과라는 값을 넣을 것
    likes.entry("아무개").or_insert("망고");//아무개가 키로 있는지 확인하고... 없으면 망고라는 값을 넣을 것

    let some_vec = vec![5, 2,6,8,2,8,6,3,2,6,2,3,2,1,5];
    let mut fre_vec:HashMap<i32, u32> = HashMap::new();

    for i in &some_vec{
        let freq: &mut = fre_vec.entry(*i).or_insert(0);
        #freq += 1;
    }
    println!("값 {:?}", fre_vec);
}
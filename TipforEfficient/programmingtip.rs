/*
효과적인 프로그래밍 기술
 */
use std::collections::HashMap;

#[derive(Debug)]
struct Person{
    name: String,
    age: u32,
}

fn persons_by_name(persons: Vec<Person>) -> HashMap<String, Person>{
    persons.into_iter().map(|p| (p.name.clone(), p)).collect()
}


fn main(){
    let cancer = true;
    let smooking = false;

    match cancer{
        true => match smooking{
            true => println!("흡연으로 인한 암"),
            false => println!("다른 원인으로 인한 암"),
        }
        false => match smooking{
            true => println!("흡연은 암을 유발할 수 있음"),
            false => println!("암 유발 효과가 낮음"),
        }
    }
    //match 내부에 match를 또 써야 할 경우..> tuple형태로 matching을 돌려보자
    //위의 것은 아래의 코드와 같은 효과가 됨 : 구조가 간단하고 보기 편함
    match (cancer, smooking) {
        (true, true) => println!("흡연으로 인한 암"),
        (true, false) => println!("다른 원인으로 인한 암"),
        (false, true) => println!("흡연은 암을 유발할 수 있음"),
        (false, false) => println!("암 유발 효과가 낮음"),
    }

    //
    let response = vec![Ok(100), Err("Client Error"), Ok(300), Err("Server Error")];
    let result: Result<Vec<_>, &str> = response.into_iter().collect();
    println!("{:?}", result);
    //위의 예시 response에서 ok만 result로 넘기고 싶다면..?

    let person_1 = Person{
        name: "아무개".to_string(),
        age: 40,
    };
    let person_2 = Person{
        name: "바다".to_string(),
        age: 30,
    };
    let person_3 = Person{
        name: "하늘".to_string(),
        age: 45,
    };
    
    let persons = vec![person_1, person_2, person_3];
    let person_hash = persons_by_name(persons);
    for (name, details) in &person_hash{
        println!("사람 {:?} 는 {:?}",  name, details);
    }
    //거꾸로 출력하고 싶을 때
    for i in 9..0{
        println!("{}", i);
    }
    //위와 같이 처리하면 단 한줄도 출력되지 않음
    for i in (0..10).rev(){
        println!("{}", i);
    }
    //위와 같이 처리할 것...
    //또한 가장 마지막 숫자까지 출력하고 싶다면....?

    for i in 0..=10{
        println("{}", i);
    }
    //위와 같이 ..=으로 처리하기
}
/*
Option Enum
 */
/*
enum Option<T>{
    None,
    Some(T),
}
 */

struct Person{
    name: String,
    age: i32,
}

fn square(num: Option<i32>) -> Option<i32>{
    match num{
        Some(number)=> Some(number* number),
        None => None,
    }
}
fn main(){
    let mut disease = None;
    disease = Some(String::from("Diabets"));

    match disease{
        Some(disease_name) => println!("병의 이름은 {}", disease_name);
        None => println!("아무런 병이 없습니다");
    }
    let s1 = Some("무언가 옵션");
    println!("상태는 : {}, 그 값 자체는 {}", s1, s1.unwarp());

    let f1 = Some(10.54);
    let mut f2 = 16.5;
    f2 = f2 + f1.unwarp();
    println!("더한 값 : {}", f2);

    let v1 = Some(vec![0,2,3,6]);
    
    let p1 = Person{
        name: String::from("아무개"),
        age: 30,
    };
    //구조체도 option enum으로 만들 수 있음...
    let someone = Some(p1);

    let number = Some(6);
    if square(number) != None{
        println!("값은 {:?}", square(Some(6).unwarp()));
    } else{
        println!("제공된 값이 없음");
    }
    square(None);

}
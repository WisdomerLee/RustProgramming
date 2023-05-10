/*
As_ref()
partial move in option

 */
fn main(){
    //변수 Some은 값의 owner가 됨 
    // let some_option = Some("Alice".to_owned());
    // match some_option {
    //     Some(inner_value) => println!("이름 {}", inner_value),
    //     None => println!("아무 이름도 입력되지 않음"),
    // }

    // println!("{:?}", some_option);
    //위의 경우는 Some의 값이 some_option의 변수가 아닌 Some에 종속되어 있어(ownership)
    //출력을 할 수 없는 상태가 됨...
    //아래와 같이 match의 Some내부에 ref를 붙이거나...
    // let some_option = Some("Alice".to_owned());
    // match some_option {
    //     Some(ref inner_value) => println!("이름 {}", inner_value),
    //     None => println!("아무 이름도 입력되지 않음"),
    // }

    // println!("{:?}", some_option);
    //혹은
    //아래와 같이 match로 받아올 때 reference로 받아오도록 처리하면...
    // let some_option = Some("Alice".to_owned());
    // match &some_option {
    //     Some(inner_value) => println!("이름 {}", inner_value),
    //     None => println!("아무 이름도 입력되지 않음"),
    // }

    // println!("{:?}", some_option);
    //혹은
    
    let some_option = Some("Alice".to_owned());
    let some_1 = &some_option;
    let some_2 = some_option.as_ref();
    //as_ref함수로 접근하여 넘기면 ownership의 변경 없이 처리할 수 있음
    try_me(some_option.as_ref());
    println!("{:?}", some_option);

}

fn try_me(option_name: Option<&String>){
    match option_name {
        Some(inner_value) => println!("이름 {}", inner_value),
        None => println!("아무 입력도 없음"),
    }
}
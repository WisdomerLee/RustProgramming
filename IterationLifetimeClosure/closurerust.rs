/*
Closure
: 일종의 한 줄로 된 함수 같은 것..
|...| {...}
 */
fn main(){
    let x = 5;
    //아래는 특별한 입력이 없는 closure
    //만약 같은 이름과 입력 형태가 같은 closure를 같이 선언하게 되면...
    let square = || println!("x의 제곱은 {}", x*x);
    
    square();
    //특별한 입력이 있는 closure
    let square = |num: i32| println!("x의 제곱은 {}", num*num);
    let square = |num: i32| println!("큐브x의 부피는 {}", num * num * num);
    square(x);

    let y = 15;
    square(y);

    let print_info = |general_info: String, name: &str, age: i32| println!("{} {} {}", general_info, name, age);
    let (person_name, person_age) = (String::from("아무개"), 38);
    println!(general_info, &person_name, person_age);

    let squarenum = |num: f64| num*num;
    let x = 5;
    square(5);
    let y = 105.5;
    square(y);

    let division_status = |y: f32|{if y != 0.0 {true} else {false}};

    division(5.0, 10.0, division_status);
    division(54.0, 0.0, division_status);


    // let some_closure1 = |x: u32| -> u32{x+1};
    // let some_closure2 = |x| {x+1};
    let mut vec_1 = vec![2,3,5];
    let mut some_closure = || {
        vec_1.push(35);
    };
    println!("vec 1: {:?}", vec_1);
    some_closure();
    vec_1[1] = 15;

}

fn division<F: Fn(f32) -> bool>(x:f32, y:f32, f:F){
    if f(y) == true{
        println!("나누기 결과는 {}", x/y);
    }
    else{
        println!("나누기 불가능");
    }
}


/*
함수 타입

함수 자체도 입력으로 쓸 수 있음

 */

fn max(x: i32, y: i32) -> i32{
    if x>y{x} else {y}
}

fn min(x: i32, y: i32) -> i32{
    if x<y {x} else {y}
}
fn print_name(name: &str){
    println!("이름 {}", name);
}
fn print_all_info(f: fn(&str), some_one:&str, age: i32){
    f(some_one);
    println!("나이는 {}",age);
}

fn add_one(x: i32) -> i32{
    x+1
}

fn do_twice(f: fn(i32)-> i32, arg: i32) -> i32{
    f(arg) + f(arg)
}

fn main(){
    let mut f = max;
    println!("두 값중 큰 값은 {}", f(3,5));

    let (my_name, my_age) = (String::from("아무개"), 33);
    print_all_info(print_name, &my_name, my_age);

    let answer = do_twice(add_one, 5);
    println!("답은 {}", answer);
}




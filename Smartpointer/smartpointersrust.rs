/*
Smart Pointers
pointer 기능에 이어 meta data를 갖고, capacity를 추가로 가짐...?
reference counting smart pointer
string, vector : smart pointer를 쓰는 데이터 구조임

box smart pointer
Box smart pointer는 언제 쓰느냐??
 */
//아래의 enum은 자기 자신을 호출할 수 있으므로 rust에서는 enum에 할당할 값의 크기를 알 수 없어 컴파일러에서 에러를 띄움...
//enum List{
//    Cons(i32, List),
//    Nil,
//}

#[derive(Debug)]
enum List{
    Cons(i32, Box<List>),
    Nil,

}

use List::{Cons, Nil};

fn main(){
    //Box::new();로 하면 box의 smart pointer를 쓰는 것... heap에 할당
    let single_value = Box::new(0.625);
    let x = 0.625;
    println!("두 값이 같은가? {}", x == *single_value);

    let mut stack_var = 4;
    let stack_ref = &stack_var;

    let heap_var = Box::new(stack_var);

    let heap_var = Box::new(stack_ref);

    stack_var = 5;
    println!("stack 값 {}, heap 값 {}", stack_var, heap_var);

    //tuple의 형태로도 넣을 수 있음
    let point = Box::new((100, 125));

    println!("값 {} {}", point.0, point.1);
    //일반적으로 pointer의 데이터에 접근할 때는 반드시 *를 붙여서 pointer의 data에 접근한다는 것을 알려야 하지만
    //Box smart pointer는 그렇게 표시할 필요 없이 데이터에 접근하여 데이터를 반환해줌...


    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

}
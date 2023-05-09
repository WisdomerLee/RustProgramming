/*
Reference Counting Pointers
reference를 확인하여 reference가 더이상 한 곳도 없으면 pointer를 할당 해제하는 pointer
 */
use std::rc::Rc;

fn make_rc() -> Rc<String>{
    let s1 = Rc::new(String::from("Hello"));
    println!("pointer갯수 : {}", Rc::strong_count(&s1));

    let s2 = s1.clone();
    println!("clone 생성 후 pointer 갯수 {}", Rc::strong_count(&s1));
    s2
}


enum List{
    //Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main(){
    /*
    : Box smart pointer를 쓰게 되면 아래의 경우 c에 들어갈 a가 ownership이 없어 error를 띄우게 됨...
    let a = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
     */

    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));

    println!("a 생성 후 reference counting : {}", Rc::strong_count(&a));
    {let b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("b 생성 후 reference counting : {}", Rc::strong_count(&a));
    let c = Rc::new(Cons(3, Rc::clone(&a)));
    println!("c 생성 후 reference counting: {}", Rc::strong_count(&a));
    }
    println!("블록 끝난 후 reference counting: {}", Rc::strong_count(&a));
    //같은 데이터를 호출할 경우 rc의 clone을 이용하여 데이터를 복사하여 들고 가는 것을 주의깊게 볼 것...
    //물론 clone함수를 이용하여 복사할 수도 있으나 rc의 clone을 이용할 경우 rc 포인터의 참조를 활용하는 것이 0개가 되는 순간을 감지하기 쉬워지므로 내부함수를 되도록 활용할 것
    let s2 = make_rc();
    println!("함수 호출 후 숫자 {}", Rc::strong_count(&s2));
}
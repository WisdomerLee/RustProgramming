/*
Cons list customm defined smart pointer

custom defined smart pointer
 */
#[derive(Debug)]
enum List{
    Cons(i32, Option<Box<List>>),
}

struct MySmartPointer{value: i32}

impl MySmartPointer{
    fn new(x: i32) -> MySmartPointer{
        MySmartPointer{value: x}
    }
}
/*
trait deref{
    type target: name of type;
    fn deref(&self)-> &self::Target;
}
 */
use std::ops::Deref;

impl Dref for MySmartPointer{
    type Target = i32;
    fn deref(&self) -> &i32{
        &self.value
    }
}
/*
trait drop{
    pub trait Drop{
        fn drop(&mut self);
    }
}
 */
impl Drop for MySmartPointer{
    fn drop(&mut self){
        println!("my smart pointer오브젝트 메모리에서 해제 {:?}", self.value);
    }
}


fn main(){
    //이 방식이 이전의 List내부에 다른 Nil이라는 것을 두는 것보다 더 나은 방식, heap에 메모리를 할당할 때 더 좋은 방법이라고 함..?
    let list = List::Cons(1, Some(Box::new(List::Cons(2, Some(Box::new List::Cons(3, None))))));
    println!("{:?}", list);

    let a = 50;
    let b = &a;
    println!("{}", 50 == a);
    println!("{}", 50== *b);
    //println!("{}", a == b);

    let a = 50;
    let b = Box::new(a);
    println!("{}", 50 == a);
    println!("{}", 50== *b); // deref trait
    //println!("{}", a == b);

    let sptr1 = MySmartPointer::new(a);
    let sptr2 = MySmartPointer::new(*b);
    println!("{}", a == *sptr1); 

    let sptr1 = MySmartPointer::new(a);
    let sptr2 = MySmartPointer::new(a+3);
    let sptr3 = MySmartPointer::new(a+6);


    //smart pointer는 drop 함수를 넣지 않아도 자동으로 할당이 해제되는 형태를 띔...
    //하지만 임의로 할당을 해제하게 할 수도 있음 > drop function을 넣어 호출하면 됨
    //임의로 할당을 해제하지 않으면 생성된 것의 역순으로 할당이 해제됨...
    drop(sptr1);
}
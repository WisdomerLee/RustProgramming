/*
Cons list customm defined smart pointer

custom defined smart pointer : Generic으로 바꾸기
deref coercion

 */
#[derive(Debug)]
enum List{
    Cons(i32, Option<Box<List>>),
}

struct MySmartPointer<T : std::fmt::Debug>{value: T}

impl<T: std::fmt::Debug> MySmartPointer<T>{
    fn new(x: T) -> MySmartPointer<T>{
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

impl<T: std::fmt::Debug> Deref for MySmartPointer<T>{
    type Target = T;
    fn deref(&self) -> &T{
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
impl<T: std::fmt::Debug> Drop for MySmartPointer<T>{
    fn drop(&mut self){
        println!("my smart pointer오브젝트 메모리에서 해제 {:?}", self.value);
    }
}
fn my_fn(str: &str){
    println!("main에서 받은 문자 {}", str);
}


fn main(){
    let sptr1 = MySmartPointer::new("아무개");
    my_fn(&sptr1); // &mysmartpointer -> &string -> &str

    let somevec = MySmartPointer::new(vec![1,2,3]);
    for z in &*somevec{
        println!("값 {}", z);
    }
}
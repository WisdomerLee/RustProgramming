/*
스레드간 mutex 데이터 넘기기?!
Atomic rerefence cell pointer??를 이용

 */
use std::sync::Mutex;
use std::thread;
//use std::rc::Rc;
use std::sync::Arc;

struct MyString(String);
impl MyString{
    fn new(s: &str) -> MyString{
        MyString(s.to_string())
    }
}

fn main(){
    //스마트 포인터를 적용하여 : borrow로 인해 데이터에 접근을 할 수 없는 상태들을 막아보기...
    //
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move||{
            let mut num = counter.lock().unwrap();
            *num+=1;
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("값 {}", *counter.lock().unwarp());


    let mut threads = Vec::new();
    let name = Arc::new(MyString::new("러스트"));
    for i in 0..5{
        let some_str = name.clone();
        let t = thread::spawn(move || {
            println!(" {}는 {}", some_str.0, i);
        });
        threads.push(t);
    }

    for t in threads{
        t.join().unwrap();
    }

}
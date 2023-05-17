/*
여러 스레드
스레드와 ownership
 */
use std::thread;
fn main(){
    let mut thread_vec = vec![];
    for i in 0..10{
        //밸류의 ownership을 thread로 옮겨 thread에서 변수가 살아있도록 ...
        thread_vec.push(thread::spawn(move ||{
            println!("스레드 숫자 {}", i);
        }));
    }
    //main 스레드에서 다른 스레드에서 실행되는 것들이 완료될 때까지 대기해주어야 함..
    for i in thread_vec {
        i.join();
    }
    //병렬로 실행하면 처리되는 순서는 제각각임

    let v = vec![1,2,3];
    let x = 5;
    let handle = thread::spawn(move ||{
        println!("벡터 {:?}", v);
        println!("변수 {:?}", x);
    });
    //만약 소유권이 넘어간 상태에서 원본이 사라지고... 별도 스레드에서 실행을 대기하면 어떻게 될까....?
    //primitive는 복사로 넘어가므로 >> drop으로 호출하고도 별도 스레드에서 실행이 진행되므로 살아있고
    //reference로 넘어가는 것들은 drop으로 사라지게되면 원본 자체가 사라지게 되므로 날아가게 됨....
    drop(x);
    handle.join();
}
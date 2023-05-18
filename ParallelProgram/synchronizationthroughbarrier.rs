/*
Synchronization through barriers
barrier : lock과 유사하나 lock은 오브젝트를 잠그는 것에 비해
barrier는 thread 자체를 대기시킴...?

 */

use std::thread;
use std::sync::{Arc, Mutex, Barrier};

fn main(){
    let mut threads= Vec::new();
    let barrier = Arc::new(Barrier::new(5));

    for i in 0..10{
        let barrier = barrier.clone();
        let t = thread::spawn(move|| {
            println!("기다리기 전 {}", i);
            //barrier로 대기 상태를 만들 때 이슈가 발생할 수 있음 : deadlock과 같은 경우 등..
            barrier.wait();

            println!("기다린 후 {}", i);
        });
        threads.push(t);
    }

    for t in threads{
        t.join().unwrap();
    }

    let mut thrreads = Vec::new();
    let barrier = Arc::new(Barrier::new(3));

    let data = Arc::new(vec![
        vec![1,2,3,4,5,6],
        vec![1,2,3,4,5,6],
        vec![1,2,3,4,5,6],
    ]);

    let result = Arc::new(Mutex::new(0));

    for i in 0..3{
        let barrier = barrier.clone();
        let data = data.clone();
        let result = result.clone();
        let t = thread::spawn(move ||{
            let x = data[i][0..3].iter().sum();
            *result.lock().unwrap() += x;//해당 줄이 실행되면 lock이 실행되었다가 데이터 변경으로 잠금이 풀림...
            println!("스레드 {} 부분은 완료 ", i);
            barrier.wait();
            let x:i32 = data[i][3..6].iter().sum();
            *result.clone().unwrap() += x;

            println!("스레드 {} 완료", i);
        });
        threads.push(t);

        for t in threads{
            t.join().unwrap();
        }

    }
    println!("최종 결과 {}", *result.lock().unwrap());
}
/*
thread parking

park로 스레드를 대기 시킬 수 있고 unpark로 스레드를 재개시킬 수도 있음
또한 unpark가 아닌 시간으로 대기시킬 수도 있음...

 */

use std::thread;
use std::time::Duration;
fn main(){
    let job_1 = thread::spawn(||{
        println!("첫번째 작업 시작");
        println!("두번째 작업 완료 대기");
        thread::park();
        //thread::park_timeout(Duration::from_secs(2)); //이렇게 처리하면 스레드는 2초 기다렸다가 다시 작업을 시작함 물론 unpark를 할 경우에도 다시 작업이 재개됨
        //thread::sleep(Duration::from_secs(2)); //이 경우도 똑같이 2초 대기했다가 작업이 시작됨 위와 차이점은 이 경우는 스레드의 동작 멈추는 시간을 중간에 깰 수 없다는 것 : 무조건 해당 시간을 채우고 나서야 스레드 작업이 재개됨
        //thread::yield_now();다른 스레드가 작업이 종료될 때까지 대기했다가 종료즉시 스레드 재개가 시작되어 스레드 완료가 됨
        println!("첫번째 작업 재개");
        println!("첫번째 작업 끝");
    });
    let job_2 = thread::spawn(||{
        println!("두번째 작업 시작");
        println!("두번째 작업 끝");
    });
    job_2.join().unwrap();
    println!("두번째 작업 완료");
    println!("첫번째 작업 다시 시작");
    job_1.thread().unpark();
    job_1.join().unwrap();
}
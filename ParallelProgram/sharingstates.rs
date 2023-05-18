/*
스레드간 상태 공유
데이터를 공유하게 되려면 하나의 스레드에서 데이터에 접근할 동안 다른 스레드에서 데이터 접근을 막아야 함> 특히나 변경할 데이터들 같은 경우에 특히 
 */

use std::sync::Mutex;
fn main(){
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 10;
    }

    println!("m = {:?}", m);

    //아래의 코드를 보게 되면...
    // let mut num = m.lock().unwrap();
    // let mut num1 = m.lock().unwrap();
    
    // *num = 10;
    // *num1 = 15;
    //위와 같은 코드로 짜게 되면 프로그램이 중간에 멈추게 됨 : 한 없이 대기하는 상태로 멈춤
    //첫번째 lock이 풀리기 전에 두번째로 lock이 걸려 첫번째도, 두번째도 둘 다 데이터에 접근할 수 없는 상태로 잠기는 상태가 됨 : deadlock상태
    //아래와 같이 처리하면 dead lock을 막을 수 있음
    let mut num = m.lock().unwrap();
    *num = 10;
    drop(num);
    let mut num1 = m.lock().unwrap();
    *num1 = 15;
    drop(num1);

}
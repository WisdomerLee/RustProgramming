/*
Channel을 통한 Communication
데이터를 다른 스레드로 안전하게 넘기는 방법!! : 병렬 처리에서 매우 중요한 개념
스레드에서 다른 스레드로 보내는 것을 메시지라고 하는데 이 메시지는 queue와 같은 데이터 구조를 가지고 있음
 */

use std::thread;
use std::sync::mpsc;
fn main(){
    let (tx, rx) = mpsc::channel();
    //만약 스레드에서 받는 것을 새로운 변수로 정의하면 > ownership이 넘어가고... 메시지를 받을 수 없는 상태가 됨....?
    //let rx1 = rx;
    let t = thread::spawn(move||{
        //변수의 형태가 primitive와 non-primitive인 경우 차이가 있음 : ownership에서
        let val = String::from("보내는 메시지");
        println!("스레드에서 메시지 보냄");
        tx.send(val).unwrap();
        //이것 다음은 메인 스레드에 있는 출력보다 먼저 나오거나 나중에 나옴
        println!("이것은 아마도 메인에서 메시지 받고 출력된 다음에 출력될 예정");
        //아래에서 변수의 ownership이 다른 스레드로 넘어간 상태에서 값을 출력하려고 하면 primitive는 문제 없이 출력되고 non-primitive타입은 출력할 수 없는 상태가 됨
        //value, reference의 차이
        //println!("값 {:?}", val);
    });

    // let received = rx.recv().unwrap();

    // println!("받은 메시지 {:?}", received);
    //만약 먼저 스레드가 처리될 때까지 대기했다가 넘어가면..?
    t.join();
    let mut received_status = false;
    while received_status != true{
        match rx.try_recv() {
            Ok(received_value) => {
                println!("받은 값 {:?}", received_value);
                received_status = true;
            },
            Err(_) => println!("받기 실패"),
        }
    }
    //t.join이 앞에 있게 되면 Err부분이 처리되지 않고 바로 완료쪽으로 넘어가게 됨
}
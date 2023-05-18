/*
여러 메시지 보내기
여러 프로세스 실행하기
스레드와 함수

 */
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

//함수에서 스레드를 쓴 논리를 처리하게 되면 어떻게 될까??
fn timer(d: i32, tx: mps::Sender<i32>){
    thread::spawn(move || {
        println!("{} 보냄", d);
        tx.send(d).unwrap();
    });
}


fn main(){

    let (tx, rx) = mpsc::channel();
    let t = thread::spawn(move||{
        let my_vec = vec![1,2,3,4,5];
        for i in my_vec{
            tx.send(i).unwrap();
        }
    });

    // for recieved_vals in rx{
    //     println!("받은 메시지 {}", recieved_vals);
    // }
    //순서대로 받아서 vector형태로 만들게 된다면??
    let recieved_vals_vec = rx.iter().collect::<Vec<i32>>();
    println!("받은 메시지 벡터 {:?}", recieved_vals_vec);

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move||{
        let my_vec = vec![1,2,3,4,5];
        for i in my_vec{
            tx.send(i).unwrap();
            //뭔가 긴 연산을 1초 대기로 대체
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move||{
        let my_vec = vec![6,7,8,9,10];
        for i in my_vec{
            tx1.send(i).unwrap();
            //뭔가 긴 연산을 1초 대기로 대체
            thread::sleep(Duration::from_secs(1));
        }
    });
    //각각의 스레드에서 보내는 데이터는 순차적으로 오나 (queue 데이터 형태)
    //스레드끼리의 연산 순서는 뒤섞이게 됨 1,6,2,7,8,3,등의 식으로 섞임
    for recieved_vals in rx {
        println!("받은 값 {:?}", recieved_vals);
    }

    let (tx, rx) = mpsc::channel();
    for i in 0..5{
        timer(i, tx.clone());
    }

    for recieved_vals in rx{
        println!("{} 받음", recieved_vals);
    }
    //channel에서 sender가 살아있으면 receiver도 살아있음(메시지 보낼 것을 받을 준비를 계속 하고 있게됨)
    
}
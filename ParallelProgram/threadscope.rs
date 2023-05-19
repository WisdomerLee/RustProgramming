/*
thread scope:
 */

use std::thread;
fn main(){
    let mut vec = vec![1,2,3];
    let mut x = 0;

    thread::scope(|some_scope|{
        //scope를 나누어 reference를 borrow하게 되면 scope이 다른 부분에서는 이전 scope에서 mutable로 빌리거나 immutable로 빌리거나 상관없이 mutable, immutable로 빌릴 수 있음
        some_scope.spawn(||{
            println!("첫번째 범위");
            println!("{:?}", vec);
        });

        some_scope.spawn(||{
            println!("두번째 범위");
            x +=45;
            //vec.push(4); //borrow로 인해 변경 불가...
        });
    });

    println!("스레드 완료");
    vec.push(5);
    println!("x {:?}, vec {:?}", x, vec);
}
/*
Avoiding Allocations
 */
use std::mem;
use std::mem::swap;
#[derive(Debug)]


enum Customer{
    new{name: String},
    loyal{name: String},
    rich{name: String},
}

fn promote(user: &mut Customer){
    use Customer::*;

    //아래의 clone을 이용하게 되면 memory에서 allocation이 발생하게 됨
    // *user = match user {
    //     Customer::new{name} => Customer::loyal{name: name.clone()},
    //     Customer::loyal {name} => Customer::rich{name: name.clone()},
    //     Customer::rich{name} => return,
    // }
    //mem::take를 쓰게 되면 해당 string의 enum만 바뀌게 됨..
    //
    // *user = match user {
    //     Customer::new{name} => Customer::loyal{name: mem::take(name)},
    //     Customer::loyal {name} => Customer::rich{name: mem::take(name)},
    //     Customer::rich{name} => return,
    // }
    //아래와 같은 형태가 된다면...?
    //take와 replace모두 둘 다 변수를 바꿔치기한다는 점에서 같으나 ..
    //take는 기존 값을 돌려주고 기존 값을 기본 값으로 바꿔치기 함
    //replace는 기존 값을 돌려주나 기존 값을 다른 입력된 값으로 바꾸어줌
    

    *user = match user {
        Customer::new{name} => Customer::loyal{name: mem::replace(name, String::new())},
        Customer::loyal {name} => Customer::rich{name: mem::replace(name, String::new())},
        Customer::rich{name} => return,
    }
}
fn main(){
    let mut customer_1 = Customer::new{name:"아무개".to_string()};
    promote(&mut customer_1);
    println!("손님 1 {:?}", customer_1);

    promote(&mut customer_1);
    println!("손님 1 {:?}", customer_1);

    let mut s1 = "변수하나".to_string();
    let mut s2 = "변수둘".to_string();

    let temp = s1;
    s1 = s2;
    s2 = temp;
    println!("s1 : {}, s2: {}", s1, s2);

    swap(&mut s1, &mut s2);
    println!("s1 : {}, s2: {}", s1, s2);
}
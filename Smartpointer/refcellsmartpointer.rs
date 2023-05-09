/*
RefCell : rust의 borrow rule을 잘 따르고 있다고 가정하면 실행 중에 borrow rule을 체크하는 것
기본적으로 rust는 compile 시점에서 borrow를 판단..

 */

use std::cell::RefCell;
use std::rc::Rc;

fn main(){
    /*
    아래의 예시는 mutable borrow, immutable borrow가 동시에 있어(함수 등에 쓰이기 전에 변수들이 계속 선언되어) rust의 borrow 법칙에 어긋남..
    그래서 컴파일 단계에서 에러를 발생시킴
    let mut x = 50;
    let x1 = &x;
    let x2 = &x;
    let x3 = &mut x;
    println!("{} {}", x1, x2);


    하지만 RefCell에서는 borrow rule을 컴파일 단계가 아닌 실행 단계에서 점검함...

     */


    let a = RefCell::new(10);

    let b = a.borrow();
    let c = a.borrow();
    //b와 c의 메모리 할당 해제
    drop(b);
    drop(c);

    let mut d = a.borrow_mut();

    //drop(d);
    println!("{:?}", a);

    //혹은 아예 block을 나누어 자동으로 borrow를 drop 시켜도 됨
    let a = RefCell::new(10);

    {
        //b와 c는 코드 블록이 분리되어 블록이 끝나는 순간 사라지게 되므로 d와 동시에 존재할 수 없음...> 실행 단계에서 borrow 법칙에 어긋나지 않게 됨..
        let b = a.borrow();
        let c = a.borrow();
    }

    let mut d = a.borrow_mut();
    drop(d);
    println!("{:?}", a);

    /*
    다음과 같은 경우는 rust의 프로그램에서 있을 수 없음..
    기본 변수 자체가 수정 불가능한 변수로 선언되어 수정 가능한 빌려옴이 안 됨...
    let x = 32;
    let x1 = &mut x;
    */
    
    //RefCell에서는 과연?
    //변수 자체는 수정 불가능한 변수로 선언하고.
    let a = RefCell::new(10);
    //빌려오는 단계에서 수정 가능하게 빌려오고
    let mut b = a.borrow_mut();

    //변경을 시도
    *b = 15;
    //빌려온 변수를 삭제
    drop(b);
    //원본을 출력하면?
    println!("{:?}", a);
    //변경된 값이 a에 저장됨!?
    /*
    아니면 그냥 직접적으로 수정불가능한 변수를 수정 가능하게 빌려온 상태에서 바로 수정하려고 할 수도 있음
    *a.borrow_mut() = 15;
     */


    //RefCell로 만든 것을 Rc스마트 포인터로 만들고 
    let a = Rc::new(RefCell::new(String::from("자바")));
    //복사하여 빌린 뒤에
    let b = Rc::clone(&a);
    //수정 가능하게 변환한 다음
    *b.borrow_mut() = String::from("C++");
    //원본을 출력하면? : 일반적인 경우엔 borrow된 상태이므로 출력이 제대로 안 되지만..
    //Rc로도 묶여있으므로 
    println!("{:?}", a);

}
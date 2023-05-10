/*
Reference Cycle :
 */

use std::borrow::Borrow;
use std::cell::RefCell;
ust std::rc::{Rc, Weak};
//Rc에서는 ownership이 넘어가고 등등의 현상이 있으나..
//Weak : ownership과 무관하게 reference 참조를 가져오게 된다고 함...


#[derive(Debug)]
struct Node{
    next : Option<Weak<RefCell<Node>>>,
}

impl Drop for Node{
    fn drop (&mut self){
        println!("{:?}없애기", self);
    }
}

struct Nod{
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main(){
    /*
    Node의 next가 Option<Rc<RefCell<Node>>>일 경우
    let a = Rc::new(RefCell::new(Node{next: None}));
    println!("a 포인터 직접 참조 갯수 {:?}", Rc::strong_count(&a));

    let b = Rc::new(RefCell::new(Node{next: Some(Rc::clone(&a))}));
    println!("b 생성 뒤 a 포인터 참조 갯수 {:?}", Rc::strong_count(&a));
    println!("b 포인터 참조 갯수 {:?}", Rc::strong_count(&b));

    let c = Rc::new(RefCell::new(Node{next: Some(Rc::clone(&b))}));
    //순환 참조가 일어나게 된다면....? 
    (*a).borrow_mut().next = Some(Rc::clone(&c)); //순환 참조가 일어나지 않으면 변수들은 drop되게 됨..
    //Rc smart pointer에서 순환참조가 일어나게 되더라도 해결책이 있다고 함...


    println!("c가 생성되고 a가 c를 참조하게 된다면 생성 뒤 a 포인터 참조 갯수 {:?}", Rc::strong_count(&a));
    println!("b 포인터 참조 갯수 {:?}", Rc::strong_count(&b));
    println!("c 포인터 참조 갯수 {:?}", Rc::strong_count(&c));

     */
    

    let a = Rc::new(RefCell::new(Node{next: None}));
    println!("a 포인터 직접 참조 갯수 {:?}, 간접 참조 갯수 {:?} ", Rc::strong_count(&a), Rc::weak_count(&a));

    //Rc의 참조와는 달리 Rc::clone대신 Rc::downgrade로 참조를 빌려옴 : ownership을 가져오진 않고 a를 참조함 >> ownership을 가져오진 않으므로 변경이 안됨..
    //하지만 memory 누수를 완전히 막을 순 없음...
    let b = Rc::new(RefCell::new(Node{next: Some(Rc::downgrade(&a))}));
    println!("b 생성 뒤 a 포인터 참조 갯수 {:?}, 간접 참조 갯수 {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("b 포인터 참조 갯수 {:?}, 간접 참조 갯수 {:?}", Rc::strong_count(&b), Rc::weak_count(&b));

    let c = Rc::new(RefCell::new(Node{next: Some(Rc::downgrade(&b))}));
    //순환 참조가 일어나게 된다면....? 
    (*a).borrow_mut().next = Some(Rc::downgrade(&c));
    


    println!("c가 생성되고 a가 c를 참조하게 된다면 생성 뒤 a 포인터 참조 갯수 {:?}, 간접 참조 갯수 {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("b 포인터 참조 갯수 {:?}, 간접 참조 갯수 {:?}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("c 포인터 참조 갯수 {:?}, 간접 참조 갯수 {:?}", Rc::strong_count(&c), Rc::weak_count(&c));
    //직접 참조 갯수는 그대로고 간접 참조 갯수가 늘어나게 됨....
    //그리고 변수들은 하나씩 메모리에서 할당 해제가 되게 됨 : 순환인데도 weak로 물렸다는 이유로 메모리 누수를 방지할 수 있음

    let leaf = Rc::new(Nod{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node{
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    

}
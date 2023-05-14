/*

    사용자가 가장 최근에 샀던 물건을 알고 싶음
Hashmaps 와 Doubly Link list를 이용
 */
use std::cell::RefCell;
use std::rc::Rc;

struct List<T>{
    head: Pointer<T>,
    tail: Pointer<T>,
}

struct Node<T>{
    element: T,
    next: Pointer<T>,
    prev: Pointer<T>,
}

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

impl <T: std::fmt::Display> Node<T>{
    fn new(element: T) -> Rc<RefCell<Node<T>>>{
        Rc::new(RefCell::new(Node{
            element: element,
            prev: None,
            next: None,
        }))
    }
}

impl <T : std::fmt::Display> List<T>{
    fn new() -> Self{
        List{
            head: None,
            tail: None,
        }
    }
    fn push_front(&mut self, element: T){
        let new_head = Node::new(element);
        match self.head.take(){
            Some(old_head) =>{
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }
            None =>{
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn push_back(&mut self, element:T){
        let new_tail = Node::new(element);
        match self.tail.take() {
            Some(old_tail) =>{
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail.clone());
                self.tail = Some(new_tail);
            }
            None =>{
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }
    fn remove_front(&mut self){
        if self.head.is_none(){
            println!("비어있음");
        }else{
            self.head.take().map(|old_head|{
                match old_head.borrow_mut().next.take(){
                    Some(new_head) =>{
                        new_head.borrow_mut().prev.take();
                        self.head = Some(new_head);
                        self.head.clone()
                    }
                    None =>{
                        self.tail.take();
                        println!("이미 비어있음");
                        None
                    }
                }
            });
        }
    }
    fn remove_back(&mut self){
        if self.tail.is_none() {
            println!("이미 비어있음");
        } else{
            self.tail.take().map(|old_tail|{
                match old_tail.borrow_mut().prev.take() {
                    Some(new_tail) =>{
                        new_tail.borrow_mut().next.take();
                        self.tail = Some(new_tail);
                        self.tail.clone()
                    }
                    None =>{
                        self.head.take();
                        println!("리스트 원소 0개");
                        None
                    }
                }
            });
        }
    }
    fn print(&self){
        if self.head.is_none(){
            println!("[]");
            return;
        } else {
            let mut traversal = self.head.clone();
            while !traversal.is_none() {
                println!("{}", traversal.as_ref().unwrap().borrow().element);
                traversal = traversal.unwrap().borrow().next.clone();
            }
            println!();
        }
    }

    fn move_to_tail(&mut self, node: &Rc<RefCell<Node>>){
        let prev = node.borrow().prev.as_ref().map(|a| Rc::clone(a));
        let next = node.borrow().next.as_ref().map(|a| Rc::clone(a));
        match (prev, next){
            (None, None) =>{

            }
            (Some(_), None) => {

            }
            (None, Some(next)) => {
                node.borrow_mut().next = None;
                next.borrow_mut().prev = None;
                self.head = Some(next.clone());

                let prev_tail = self.tail.as_res().unwrap();
                prev_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(prev_tail.clone());
                self.tail = Some(node.clone());
            }
            (Some(prev), Some(next)) =>{
                node.borrow_mut().next = None;
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev.clone());

                let prev_tail = self.tail.as_ref().unwrap();
                prev_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(prev_tail.clone());
                self.tail = Some(node.clone());
            }

        }
    }
}
#[derive(Debug)]
struct MRP_Item{
    map : HashMap<i32, Rc<RefCell<Node>>>,
    item_list: List,
    size: i32,
    capacity: i32,
}

impl MRP_Item{
    fn new(capacity: i32) -> Self{
        Self{
            map: HashMap::new(),
            item_list: List::new(),
            size: 0,
            capacity: capacity,
        }
    }
    fn purchased(&mut self, prod_id: i32){
        if let Some(node) = self.map.get(&prod_id){
            self.item_list.move_to_tail(node);
        } else{
            if self.size >= self.capacity{
                let prev_head = self.item_list.remove_front().unwrap();
                self.map.remove(&prev_head.unwrap().borrow().prod_id);
            }
            let node = self.item_list.push_back(prod_id).unwrap();
            self.map.insert(prod_id, node);
            self.size +=1;
        }
    }

    fn print(&self){
        let mut traversal = self.item_list.head.clone();
        while !traversal.is_none(){
            let temp = traversal.clone().unwrap();
            println!("{}", temp.borrow().prod_id);
            traversal = temp.borrow().next.clone();
        }
        println!("{}", traversal)
    }
}

fn main(){
    let mut items_list = MRP_Item::new(3);
    items_list.purchased(10);
    items_list.print();

    items_list.purchased(25);
    items_list.print();
    items_list.purchased(3);
    items_list.print();
}
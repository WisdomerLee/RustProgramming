/*
Link List
singly link list...
 */
#[derive(Debug)]
struct linklist<T: std::fmt::Debug + std::marker::Copy>{
    head: pointer<T>,
}
struct Node<T: std::fmt::Debug + std::marker::Copy>{
    element: T,
    next:pointer<T>,
}
type pointer<T> = Option<Box<Node<T>>>;

impl<T: std::fmt::Debug + std::marker::Copy> linklist<T>{
    fn create_empty_list() -> linklist<T>{
        linklist { head: () }
    }
    fn add(&mut self, element:T){
    //이렇게도 할 수 있지만...
    //     match self.head{
    //         None => {self.head = Some(Box::new(Node{element: element, next: None}))
    //     }
    //     Some(previous_head) => {
    //         let new_head = Some(Box::new(Node{
    //             element: element, next: Some(previous_head)
    //         }));
    //         self.head = new_head;
    //     }
    // }

    let previous_head = self.head.take();

    let new_head = Box::new(Node{
        element: element, next: previous_head,
    });
    self.head = Some(new_head);
    }
    fn remove(&mut self) -> Option<T>{
        let previous_head = self.head.take();
        match previous_head{
            Some(old_head) =>{
                self.head = old_head.next;
                Some(old_head.element)
            }
            None => None
        }
    }
    fn peek(&self) -> Option<T>{
        match &self.head{
            Some(H) => Some(H.element),
            None => None,
        }
    }
    fn printing(&self){
        let mut list_traversal = &self.head;
        while true{
            match list_traversal{
                Some(Node) => {
                    println!("{:?}", Node.element);
                    list_traversal = &Node.next;
                }
                None => break
            }
        }
    }
}

fn main(){
    let list= Node{element:1, next:None};
    let list = Node{element:1, next:Some(Box::new(Node{element:2, next:Some(Box::new(Node{
        element:3, next:None
    }))
}))};
    let list = linklist{head: Node{element: 1, next: None}};
    let list = linklist{head: Node{element:1, next:Some(Box::new(Node{
        element:2, next: Some(Box::new(Node { element: 3, next: None }))
    }))}};

    let list = linklist{head:None};
    let list = linklist{head:Some(Box::new(Node{element:100, next: Some(Box::new(Node { element: 200, next: None }))}))};

    println!("{:?}", list.head.unwrap().element);
    println!("{:?}", list.head.unwrap().next.element);

    let mut list = linklist::create_empty_list();
    list.add(5);
    list.add(7);
    list.add(10);
    list.add(15);
    list.add(25);
    println!("현재 list 목록 {:?}", list);

    list.remove();
    println!("현재 list 목록 {:?}", list);

    println!("peek {}", list.peek());

    list.printing();

}
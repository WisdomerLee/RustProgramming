/*
fetching top product
서로 다른 국가에서 가장 잘 팔리는 물건들을 얻을 것임


Linklist와 iterators를 활용
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

    fn reverse(&mut self){
        if self.head.is_none() || self.head.as_ref().unwrap().next.is_none(){
            return;
        }
        let mut prev = None;
        let mut current_node = self.head.take();
        while current_node.is_some() {
            let next = current_node.as_mut().unwrap().next.take();
            current_node.as_mut().unwrap().next = prev.take();
            prev = current_node.take();
            current_node = next;
        }
        self.head = prev.take();
    }
}

fn sort_lists(vec_list: &mut Vec<linklist<i32>>) -> linklist<i32>{
    let mut sorted_list = linklist::create_empty_list();
    let mut values: Vec<i32> = Vec::new();
    while true{
        let values = vec_list.into_iter()
        .map(|x| x.head.as_ref().unwrap().element)
        .collect::<Vec<i32>>();

        let min_val = *values.iter().min().unwrap();
        let min_index = values.iter().position(|x| *x ==min_val).unwrap();

        sorted_list.add(min_val);
        vec_list[min_index].remove();
        if vec_list[min_index].head.is_none(){
            vec_list.remove(min_index);
        }
        if vec_list.len() == 0 {
            break;
        }
    }
    sorted_list
}

fn main(){
    let mut list1 = linklist::create_empty_list();
    list1.add(45);
    list1.add(50);
    list1.add(35);
    list1.add(32);
    list1.add(11);

    let mut list2 = linklist::create_empty_list();
    list2.add(60);
    list2.add(30);

    let mut list3 = linklist::create_empty_list();
    list3.add(53);
    list3.add(27);
    list3.add(10);

    let mut result = sort_list(&mut vec![list1, list2, list3]);
    result.printing();

    result.reverse();
    result.printing();
}
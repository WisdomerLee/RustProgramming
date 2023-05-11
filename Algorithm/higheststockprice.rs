/*
가장 높은 주식
가장 높은 주식을 알고 싶을 때...
Maxstack, structure, vector를 씀
*/

struct MaxStack{
    main_stack:Vec<i32>,
    maximum_stack: Vec<i32>,
}

impl MaxStack{
    fn new() -> Self{
        MaxStack{
            main_stack: Vec::new(),
            maximum_stack: Vec::new(),
        }
    }
    fn push(&mut self, value: i32){
        self.main_stack.push(value);
        if !self.maximum_stack.is_empty() && self.maximum_stack.last().unwrap()> &value{
            self.maximum_stack.push(*self.maximum_stack.last().unwrap());
        }
        else{
            self.maximum_stack.push(value);
        }
    }

    fn pop(&mut self){
        self.main_stack.pop();
        self.maximum_stack.pop();
    }

    fn max_value(&self) -> i32{
        *self.maximum_stack.last().unwrap()
    }
}
fn main(){
    let mut stack = MaxStack::new();
    stack.push(55);
    stack.push(70);
    stack.push(40);
    stack.push(150);
    stack.push(76);

    println!("가장 큰 값 :
    {:?}", stack.max_value());
    stack.pop();
    println!("한 주 전 가장 큰 값 :
    {:?}", stack.max_value());


}
/*
Stack : 기본적인 데이터 구조
책이 쌓여있는 것이라고 생각하면 쉬움
push로 stack에 데이터 집어넣기
pop으로 stack의 최상단 데이터 빼기

 */
fn new_stack(maxsize:usize) -> Vec<u32>{
    let vec = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<u32>)->Option<u32>{
    let poped_val = stack.pop();
    println!("The poped value is {:?}", poped_val);
    poped_val
}

fn push(stack: &mut Vec<u32>, item: u32, maxsize:usize)
{
    if stack.len()==maxsize{
        println!("더 이상 추가할 수 없음");
    } else{
        stack.push(item);
        println!("Stack: {:?}", stack);
    }
}
fn size(stack: &Vec<u32>) -> usize{
    stack.len()
}

fn input()->u32{
    let mut n = String::new();
    std::io::stdin()
    .read_line(&mut n)
    .expect("읽기 실패");
    let n = n.trim().parse().expect("잘못된 값");
    n
}

fn main()
{
    println!("첫 스택 만들기");
    println!("스택의 크기를 지정합시다");
    let size_stack = input();

    let mut stack = new_stack(size_stack as usize);

    println!("\n\n*******menu*****\n");
    println!("1.Push \n 2.Pop\n 3.Display\n, 4.Size\n5.Exit");
    println!("\nEnter your choice: ");
    let choice = input();
    match choice{
        1 =>{
            println!("Enter the value to be insert: ");
            let item = input();
            push(&mut stack, item, size_stack as usize);
        },
        2 => println!("The element which is poped is {:?}", pop(&mut stack)),
        3 => println!("The elements are {:?}", stack),
        4 => println!("The size of the stack is {}", size(&stack)),
        _=> println!("\nExiting"),
    }
    
}
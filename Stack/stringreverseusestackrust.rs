/*
Stack : 기본적인 데이터 구조
책이 쌓여있는 것이라고 생각하면 쉬움
push로 stack에 데이터 집어넣기
pop으로 stack의 최상단 데이터 빼기

 */
// fn new_stack(maxsize:usize) -> Vec<u32>{
//     let vec = Vec::with_capacity(maxsize);
//     vec
// }
fn new_stack(maxsize:usize)-> Vec<char>{
    let vec = Vec::with_capacity(maxsize);
    vec
}

// fn pop(stack: &mut Vec<u32>)->Option<u32>{
//     let poped_val = stack.pop();
//     println!("The poped value is {:?}", poped_val);
//     poped_val
// }
fn pop(stack: &mut Vec<char>) -> Option<char>{
    let poped_val = stack.pop();
    poped_val
}

// fn push(stack: &mut Vec<u32>, item: u32, maxsize:usize)
// {
//     if stack.len()==maxsize{
//         println!("더 이상 추가할 수 없음");
//     } else{
//         stack.push(item);
//         println!("Stack: {:?}", stack);
//     }
// }

fn push(stack: &mut Vec<char>, item: char, maxsize:usize)
{
    if stack.len()==maxsize{
        
    } else{
        stack.push(item);
        
    }
}
// fn size(stack: &Vec<u32>) -> usize{
//     stack.len()
// }
fn size(stack: &Vec<char>) -> usize{
    stack.len()
}

// fn input()->u32{
//     let mut n = String::new();
//     std::io::stdin()
//     .read_line(&mut n)
//     .expect("읽기 실패");
//     let n = n.trim().parse().expect("잘못된 값");
//     n
// }

fn main()
{
    let input_string = String::from("러스트 시작함");

    let size_stack = input_string.len();
    let mut stack = new_stack(maxsize);
    
    let mut rev_string = String::new();

    for i in input_string.chars(){
        push(&mut stack, i, size_stack);
    }

    for i in 0..size(&stack){
        rev_string.push(pop(&mut stack).unwrap());
    }
    println!("원래 문자열 {:?}", input_string);
    println!("뒤집힌 문자열 {:?}", rev_string);

}
/*
convert to postfix
evaluation expression from postfix
operator들의 우선 순위...
stack에는 연산자들을 집어넣고
postfix에는 숫자와 연산자의 우선순위가 높은 것들을 차례로 집어넣게 됨...

 */
/*
Stack : 기본적인 데이터 구조
책이 쌓여있는 것이라고 생각하면 쉬움
push로 stack에 데이터 집어넣기
pop으로 stack의 최상단 데이터 빼기

 */

fn new_stack(maxsize:usize)-> Vec<char>{
    let vec = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<char>) -> Option<char>{
    let poped_val = stack.pop();
    poped_val
}


fn push(stack: &mut Vec<char>, item: char, maxsize:usize)
{
    if stack.len()==maxsize{
        
    } else{
        stack.push(item);
        
    }
}

fn size(stack: &Vec<char>) -> usize{
    stack.len()
}


fn individual_symbols(input_expr:String) -> Vec<String>
{
    let mut tokenized_input:Vec<String> = Vec::new();
    let input_chars:Vec<char> = input_expr.chars().collect();
    let mut temp: Vec<char> = Vec::new();
    for i in input_chars{
        if i !='+' && i!='-' && i != '/' && i != '*' && i != '^' && i != '(' && i != ')'{
            temp.push(i);
            continue;
        } else{
            if temp.len() ==0{
                tokenized_input.push(i.to_string());
            } else{
                tokenized_input.push(temp.into_iter().collect());
                tokenized_input.push(i.to_string());
                temp = vec![];
            }
        }
    }
    if temp.len() != 0{
        tokenized_input.push(temp.into_iter().collect());
    }
    println!("{:?}", tokenized_input);
    tokenized_input
}

fn infix_to_postfix(intput: Vec<string>) ->Vec<string>{
    let size_expr = input.len();
    let mut stack = new_stack(size_expr);
    let mut postfix: Vec<String> = Vec::new();
    for i in input{
        match i.as_str() {
            "+"|"-"|"*"|"/"|"^" =>{
                if(size(&stack)==0){
                    push(&mut stack, i, size_expr);
                } else{
                    if priority(&i) > priority(stack.last().unwrap()){
                        push(&mut stack, i, size_expr);
                    } else{
                        while priority(&i) <= priority(stack.last().unwrap()){
                            postfix.push(pop(&mut stack).unwrap());
                            if stack.last() == None{
                                break;
                            }
                        }
                        push (&mut stack, i, size_expr);
                    }
                }
            }
            "("=>push(&mut stack, i, size_expr),
            ")"=>{
                while stack.last().unwrap() != "("{
                    postfix.push(pop(&mut stack).unwrap());
                }
                pop(&mut stack).unwrap();
            }
            _=>postfix.push(i),
        }
    }
    while size(&stack) != 0{
        postfix.push(pop(&mut stack).unwrap());
    }

    postfix
}

fn priority(x:&String)->u8{
    if("+"==x)|("-"==x){
        1
    }
    else if("*"==x)|("/"==x){
        2
    }
    else if("^"==x){
        3
    }
    else{
        0
    }
}
/*
postfix 평가
연산에 쓰일 것들을 stack에 넣음 (숫자, char등..)
연산자에 2개의 연관자가 있으면 연산을 수행하고 스택에 넣음
 */

fn postfix_evaluation(postfix: Vec<String>) ->f32{
    let size_expr = postfix.len();
    let mut result_stack = new_stack(size_expr);
    for i in postfix{
        match i.as_str(){
            "+"|"-"|"*"|"/"|"^" =>{
                let oper = i;
                let op2 = pop(&mut result_stack).unwrap();
                let op1 = pop(&mut result_stack).unwrap();
                let result = operation(op1, op2, oper);

                push(&mut result_stack, result.to_string(), size_expr);
            }
            _ =>{push(&mut result_stack, result.to_string(), size_expr);}
        }
    }
    pop(&mut result_stack).unwrap().parse::<f32>().unwrap()

}

fn operation(op1:String, op2:String, oper:String) -> f32{
    let op1 = op1.parse::<f32>().unwrap();
    let op2 = op2.parse::<f32>().unwrap();
    let result = match oper.as_str(){
        "+" => op1 + op2,
        "-" => op1 - op2,
        "*" => op1 * op2,
        "/" => op1 / op2,
        "^" => op1.powf(op2),
        _=>0.0,
    };
    result
}

fn main()
{
    let input_expr = String::from("(33+45/3*(2+9)-50)");
    println!("기본 수식 : {:?}", input_expr);
    let input_expr_tokenized = individual_symbols(input_expr);

    let postfix = infix_to_postfix(input_expr_tokenized);

    println!("계산 결과 : {}", postfix_evaluation(postfix));
    
}
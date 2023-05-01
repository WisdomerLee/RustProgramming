
// 함수 : 입력

fn main()
{
    basic_fn();
    function_with_inputs("아무개", 40000);

    let full_name = "호";
    let salary_info = 50000;
    function_with_inputs(full_name, salary_info);
    let answer = function_with_inputs_outputs(10, 15);
    println!("곱의 값 {}", answer);
    let (multiplication, addition, substraction) = function_with_inputs_multiple_outputs(5, 25);
    println!("
    곱 = {}, 더하기 = {}, 빼기 = {}
    ", multiplication, addition, substraction);
    let result = function_with_inputs_multiple_outputs(10, 15);

    println!("
    곱 = {}, 더하기 = {}, 빼기 = {}
    ", result.0, result.1, result.2);

    let full_name = {
        let first_name = "아무개";
        let last_name = "공";
        format!("{} {}", first_name, last_name)
    };

    println!("이름 {}", full_name);

    let mut n = String::new();

    std::io::stdin()
    .read_line(&mut n)
    .expect("읽기에 실패했습니다");
        let n = n.trim().parse().expect("invalid input");
    println!("{:?}", n);
    
}

//main함수 바깥에 다른 함수를 하나 정의해보자
//자주 쓰게 되는 것을 묶어 함수로 만들어 여러번 호출하면 > 코드 길이도 줄고 오류 잡기도 쉬움..
fn basic_fn()
{
    println!("기본 함수");
}

fn function_with_inputs(name: &str, salary: i32)
{
    println!("{}는 {}를 받고 있습니다", name, salary);
}

fn function_with_inputs_outputs(num1:i32, num2:i32)-> i32{
    //함수가 값을 돌려주는 것이 될 땐 돌려주는 값 뒤에 세미콜론을 붙이면 에러가 남...
    num1*num2
    
}
//여러 값을 돌려줄 땐 Tuple 형태로 반환하게 됨..
fn function_with_inputs_multiple_outputs(num1:i32, num2:i32)-> (i32, i32, i32){
    //함수가 값을 돌려주는 것이 될 땐 돌려주는 값 뒤에 세미콜론을 붙이면 에러가 남...
    (num1*num2, num1+num2, num1-num2)
    
}
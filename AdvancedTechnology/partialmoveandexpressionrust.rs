/*
Unit type
Expression vs statements
Partial Move
 */

fn f1 () ->(){}
fn f2 () {}

fn division(divident : f64, divisor: f64) -> Result<(), String> {
    let answer = match divisor{
        0.0 =>{
            println!("0으로 나눌 수 없어요!");
            Err(String::from("0으로 나누기 에러"))
        },
        _ =>{
            println!("나눌 수 있음");
            Ok(())
        },
    };
    answer
}

fn square(num:i32) -> i32{
    num*num
}
fn main(){
    let x = ();
    f1();
    f2();
    let x = {
        println!("코드 블럭에 진입!");
        println!("프로그램 계산 중");
    };
    division(9.0, 3.0);
    //statement: value를 돌려주지 않음...
    //expression: value를 돌려줌..
    let x = ();
    let y = println!("hello world");
    println!("x와 y가 같을까? {}", x==y);

    /*
    Expression vs Statements
     */

    let num1 = 10;
    //let num2 = (let x = 100); //statement라고 볼 수 있음...
    //아래의 경우는 exression으로 볼 수 있음....? : 값을 얻어올 수 있음
    //statement의 경우는 특정 값으로 얻어올 수 없음...

    let y = {
        let x = String::from("Hello rust");
        x
    };
    //함수가 값을 돌려줄 경우 expression
    //그렇지 않을 경우 statement라고 생각하면 됨...
    
    square(4);
    println!("이 것은 statement");

    //partial move : structure의 일정부분만 move로 얻어오거나 할 때 쓸 수 있음, 옮기지 않은 부분은 그대로..
    let student_1 = Student{
        name: String::from("아무개"),
        course: vec!["수학".to_string(), "지질학".to_string(), "물리학".to_string()],
        age: 28,
    };
    //여기서 partial move가 일어남... string은 copy가 되지 않고 넘어가기 때문..
    let name = student_1.name;
    let course = &student_1.course;
    let age = student_1.age;


    //아래와 같이 출력하려고 시도하면 에러가 남 : 이유는? student_1에 해당되는 것들이 다른 변수들로 지정되면서 ownership이 넘어가기 때문..>> 그렇다면....
    // println!("학생 이름 {}", student_1.name);
    // println!("학생이 배우는 과목 {}", student_1.course);
    // println!("학생의 나이 {}", student_1.age);
    //아래와 같이 시도해도 ....?일정 부분이 move로 넘어갔기 때문에 (참조를 생각해보자) 전체를 출력하는 것이 막히게 됨
    //println!("학생 {:?}", student_1);

}

#[derive(Debug)]
struct Student{
    name: String,
    course: Vec<String>,
    age: u8,
}
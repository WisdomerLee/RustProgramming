/*
lifetimes

 */

fn main(){
    //아래의 경우는 pointer가 {}내부의 것을 가리키다 사라지는 변수의 pointer를 가리키기 때문에 문제가 생김
    // let i: &i32;
    // {
    //     let j =5;
    //     i =&j;
    // }
    // println!("i 값: {}", i);
    
    //함수 자체를 reference를 참조하게 하여 해당 변수의 reference를 돌려주게 되면 문제가 없음..

    let some_int = 10;
    let additional_int = some_fn(&some_int);
    println!("{}", additional_int);
    
    
    //아래와 같이 된다면... : 일반적인 경우엔 변수의 life time이 끝난 상태이므로 종료되지만...
    //fn str_fun에서 변수의 life time을 알려주어 실제로 v는 s1이 살아있으면 호출할 수 있게 됨...?
    let s1 = "Hello";
    {
        let s2 = String::from("World");
        v= str_fun(s1, s2.as_str());
    }
    println!("{}", v);

    let int1 = 5;
    let int2 = 10;
    
}


// fn some_fn(first: &str, second: &str) -> &str{
//     first
// }

//아래의 함수의 경우 life time의 길이를 a,b의 형태가 있을 때 돌려주는 return 값은 최소한 a의 변수의 라이프타임 길이 만큼은 있어야 할 것을 알려줌 :
//reference로 돌려줄 때 lifetime을 generic type으로 'a, 'b, 등과 같이 표현하고 이를 generic을 표현하는 것처럼 씀
//
fn str_fun<'a, 'b>(firststr: &'a str, secondstr: &'b str)->&'a str{
    firststr
}
//아래의 함수는 reference가 아닌 value값을 돌려주게 됨
fn greater(i: &i32, j: &i32) -> i32{
    if i>j{
        *i
    }
    else {
        *j
    }
}

fn some_fn(i: &i32) ->&i32{
    &i
}

//함수의 lifetime을 알려줄 경우는 반드시 함수가 입력, 출력 중 한 곳을 refernece로 받아야 할 경우에 씀...
//reference가 아닌 경우에는 lifetime을 넘겨줄 수 없음!!!

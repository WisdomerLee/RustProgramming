/*
Capturing Types
 */

macro_rules! input{
    ($t: ty)=>{{
        let mut n = String::new();
        std::io::stdin()
        .read_line(&mut n)
        .expect("입력 받아오기 실패");
        //macro에서 입력받은 타입과 같은 것으로 집어넣겠다는 뜻..?!?!
        let n: $t = n.trim().parse().expect("잘못된 값");
        n
    }
    };
}
//
macro_rules! some_macro{
    //macro 외부에서 선언된 변수를 macro 내부에서 활용하려고 하면 컴파일 단계에서 에러가 발생함
    // () => {
    //     x = x + 1;
    // };
    
    ($var: ident) => {
        $var = $var + 1;
    };
}

macro_rules! create_function{
    ($func_name:ident, $input:ident, $type_input:ty, $type_output:ty) =>{
        fn $func_name($input:$type_input) ->$type_output {
            println!("You called {:?}() with the input of {:?}", stringify!($func_name), stringify!($input));
            $input
        }
    };
}

macro_rules! add_as{
    ($a: expr, $b:expr, $typ:ty) =>{$a as $typ + $b as $typ}
}

create_function!(f1, x, i32, i32);
fn main(){
    // println!("실수 숫자를 넣어봅시다");
    // let some_input = input!(f32);

    println!("{}", add_as!(15, 2.3, f32));
    // some_macro!();
    // x = x+1;
    let mut x = 4;
    some_macro!(x);
    //macro에서는 ownership을 가져오진 않음
    
    let y = f1(15);
}
/*
Declarative Macros
Basic Syntax
 */
/*
macro_rules! macro_name{
    (...) => {...};
    (...) => {...};
    (...) => {...};
}

match에서 보는 형태와 매우 흡사함...

visual studio code에서 terminal에 cargo install cargo-expand
visual studio code에서 terminal에 rustup install nightly
이것을 처리하면 rust의 확장 기능, 등을 문제없이 쓸 수 있다고 함...?

 */

macro_rules! our_macro{
    () =>{ 1+1; };
    (something 4 u) => {println!("여기서 이러시면")};
    ($e1: expr, $e2:expr) =>{
        $e1 + $e2
    };
    ($a: expr, $b:expr,$c:expr) =>{
        $a * ($b + $c)
    };

}

fn main(){
    our_macro!();
    println!("{}", our_macro!());
    our_macro!(something 4 u);

    println!("{}", our_macro!(2,2));
    println!("{}", our_macro!(5,7,3));

    //아래의 3가지 호출 모두 이상없이 동작함...?
    our_macro!();
    our_macro!{};
    our_macro![];
}
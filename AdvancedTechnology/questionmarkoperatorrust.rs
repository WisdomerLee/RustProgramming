/*
Question mark operator
 */

use std::num::ParseIntError;
#[derive(Debug)]
enum MathError{
    DivisionError_DivisionByZero,
    LogError_NonPositiveLogarithm,
    SqrtError_NegativeSquareRoot,
}
type MathResult = Result<(), MathError>;

fn parse_str(input: &str) -> Result<i32, ParseIntError>{
    //따로 Error를 처리하지 않았음에도 ?를 붙여 제대로 parsing되는 경우에는 결과를 얻고 그렇지 않으면 Error를 발생시킴
    let integer = input.parse::<i32>()?;
    
    println!("입력 {:?}은 정수 {:?}", input, integer);
    Ok(integer)
}

// fn divisor(divident: f64, divisor: f64)-> Result<f64, String>{
//     let answer = match divisor{
//         0.0 => Err(String::from("0으로 나누려고 했음")),
//         _ => Ok(divident/divisor),
//     };
//     let correct = answer?;
//     println!("에러일 때는 출력되지 않음 {:?}", correct);
//     Ok(correct)
// }



fn divisor(divident : f64, divisor: f64) -> Option<f64>{
    let answer = match divisor{
        0.0 => None,
        _=> Some(divident/divisor),
    };

    let correct = answer?;
    println!("에러일 때는 출력되지 않음 {:?}", correct);
    Some(correct)
}

fn division(x: f64, y: f64) -> MathResult{
    if y==0.0{
        Err(MathError::DivisionError_DivisionByZero)
    } else{
        println!("나누기 성공 {}", x/y);
        Ok(())
    }
}

fn sqrt(x:f64) ->MathResult{
    if x<0.0 {
        Err(MathError::SqrtError_NegativeSquareRoot)
    } else{
        println!("제곱근 성공 {}", x.sqrt());
        Ok(())
    }
}

fn ln(x:f64) -> MathResult{
    if x<=0.0 {
        Err(MathError::LogError_NonPositiveLogarithm)
    } else{
        println!("로그 성공 {}", x.ln());
        Ok(())
    }
}

fn operations(x:f64, y:f64) -> MathResult{
    //?오퍼레이터는 결과를 돌려주는 함수의 뒤 혹은 연산의 결과를 받는 변수 뒤에 쓰여서 해당 결과가 제대로 반환되면 그 값을 얻고 그렇지 않으면 Err를 띄우게 됨

    division(x,y)?;
    sqrt(x)?;
    ln(x)?;
    Ok(())
}

fn main(){
    let some_values = vec!["12", "무언가", "some(123)", "125"];
    for value in some_values{
        println!("{:?}", parse_str(value));
    }

    println!("결과는 {:?}", divisor(9.0, 3.0));
    println!("결과는 {:?}", divisor(9.0, 0.0));
    println!("결과는 {:?}", divisor(0.0, 3.0));
    
    let result = operations(0.0, 10.0);
    if result.is_ok() {
        println!("모든 함수가 성공적으로 실행됨");
    } else{
        println!("{:?}", result);
    }

}
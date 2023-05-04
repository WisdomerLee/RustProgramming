/*
Result Enum

enum Result<T, E>{
    Ok(T),
    Err(E)
}
 */

fn division(divident: f64, divisor: f64) ->Result<f64, String>{
    // if divisor == 0.0 {
    //     Err(String::from("0으로 나눔!"))
    // } else{
    //     Ok(divident/divisor)
    // }
    match divisor{
        0.0 => Err(String::from("0으로 나눔")),
        _=> Ok(divident/divisor)
    }
}
fn main(){
    println!("{:?}",  division(9.0, 3.0));
    println!("{:?}", division(4.0, 0.0));
    println!("{:?}", division(0.0, 2.0));

    let some_vec = vec![5,5,2,1,5,8];
    let result1 = match some_vec.get(5){
        Some(a) => Ok(a),
        None => Err("값이 없음"),
    };

    println!("값은 {:?}", result1);
}
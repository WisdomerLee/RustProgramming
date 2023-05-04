/*
Generics
 */

fn square<T : std::ops::Mul<Output = T> + Copy> (x:T)-> T{
    x*x //T로 만 선언하면 곱을 알 수 없음 <문자열일 경우등등.. 그래서 위에 제한 조건으로 multiply가 가능하고 그 값이 같은 데이터 타입으로 나오는 것으로 제한을 둔다고 하여 해당 일반함수로 만듦
}
/*
혹은 같은 형태의 제약을 아래와 같은 형태로도 둘 수 있음
fn square<T>(x:T) ->T
where T: std::ops::Mul<Output = T> + Copy{
    x*x
}
 */
//아래의 경우는 x,y의 데이터 타입이 같은 포인터를 일반적인 구조체로 만들었을 때
struct Point<T>
{
    x:T,
    y:T,
}
//두 데이터 타입을 다른 형태로도 만들고 싶다면...아래와 같이 선언하면 됨...
/*struct Point<T, U>{
    x:T,
    y:U,
}*/

impl<T, U> Point<T, U>
where T: std::fmt::Debug, U: std::fmt::Debug
{
    fn printing(&self){
        println!("각각의 값은 {:?} {:?}", self.x, self.y);
    }
}
fn main(){
    println!("제곱 {}", square(5));
    println!("제곱 {}", square(6.8));

}
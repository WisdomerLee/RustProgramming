/*
trait:
trait bounds
 */
trait Double{
    fn double(&self) -> Self;
}

impl Double for i32{
    fn double(&self) -> Self{
        self*2
    }
}

impl Dobule for i64{
    fn double(&self) -> Self{
        self *2
    }
}

//trait Double에서만 함수가 double이 확실히 있으나 T로 처리하게 되면 Double에 해당되는 부분인지 알 수 없으므로 double이라는 함수 자체에 접근할 수 없음..
//하지만 trait자체라는 형태로 T를 제한해주면..

fn quardruple<T: Double>(x: T) -> T{
    x.double().double()
}

fn main(){
    println!("4배 = {}", quardruple(5_i32));

}
/*
enums
아무런 값을 지정하지 않으면 차례대로 0, 1,2,순서로 숫자가 지정됨...
하지만 값을 지정하게 되면 해당 값으로 지정되고 그 이후 순차로 증가함..
 */

enum Conveyance{
    Car = 15,
    Train = 20,
    Air = 40,
}
/*
만약 enum을 아래와 같이 처리하면...
enum Conveyance{
    Car(i32),
    Train(i32),
    Air(i32),
}

main함수에서 처리한 participant부분이
let participant = Conveyance::Car(60);
이렇게 바뀌고

imple Conveyance{
    fn travel_allowance(&self) -> f32{

        let allowance = match self{
            Conveyance::Car(miles) => *miles as f32 *14.0 * 2.0,
            Conveyance::Train(miles) => *miles as f32 * 18.0 * 2.0,
            Conveyance::Ari(miles) => *miles as f32 * 30.0 * 2.0,
        };
        allowance
    }
}
또한 함수 호출이 
participant.travel_allowance()로 바뀌게 됨..
 */

impl Conveyance{
    fn travel_allowance(&self, miles: i32) -> f32{

        let allowance = match self{
            Conveyance::Car => miles as f32 *14.0 * 2.0,
            Conveyance::Train => miles as f32 * 18.0 * 2.0,
            Conveyance::Ari => miles as f32 * 30.0 * 2.0,
        };
        allowance
    }
}
#[derive(Debug)]
enum Value{
    Integer(i32),
    Float(f32),
}

fn main(){
    let participant = Conveyance::Car;
    println!("첫번째 옵션 {}", participant as i32);
    let participant1 = Conveyance::Train;
    println!("첫번째 옵션 {}", participant1 as i32);
    let participant2 = Conveyance::Air;

    println!("요금 {} {} {}", participant.travel_allowance(60), participant1.travel_allowance(60), participant2.travel_allowance(60));
    let some_val = vec![Value::Integer(12, Value::Float(15.5))];
    println!("값 : {} {}", some_val[0], some_val[1]);

    for i in some_val.iter(){
        match i {
            Value::Integer(num) => println!("해당 값 {}", num),
            Value::Float(num) =>println!("해당 값 {}", num),
        }
    }
}
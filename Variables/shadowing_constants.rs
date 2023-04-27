// 또다른 변수 shadowing, constants
// 
fn main()
{
    let (first_number, second_number) = (250, 480.5);
    let large_number = 1_000_000;
    let over_flow_number = 256;

    let n1 = 14;
    let n2 = 15.6;
    let n3 = n1 + n2 as i32;

    let s = 5*5;

    let mut p = 5;
    let p = 5*5;
    p = 60;

    let q = 32;
    let q = 'A';

    let mut r = 65;
    {
        //만약 let을 제거하고 실행하면 r값은 60으로 그냥 바뀌게 됨...
        let r = 60;
        println!("안쪽의 r값 : {}", r);
    }
    println!("바깥의 r값: {}", r);

    //immutable variable과 constant의 차이?
    //immutable variable은 mut라는 키워드로 바꾸게 만들 수 있으나 constant는 어떤 형태로도 변경이 불가능..

    const MAX_SALARY = 100_000;

}
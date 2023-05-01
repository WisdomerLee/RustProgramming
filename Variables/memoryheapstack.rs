/*
왜 primitive와 non-primitive의 =이 다르게 되는가??
memory의 특성과 연관이 있음
memory는 heap, stack, static/global, code부분으로 나뉘게 되는데

code/text, static/global, stack 영역은 어플리케이션이 실행되는 동안 메모리가 늘어날 수 없음...

static/global : 상수 등이 여기에 들어가게 됨
main함수가 불려지게 되면 함수 자체도 stack에 올라가게 되고 거기에 쓰이는 변수들은
stack에 순차적으로 올라가게 됨
데이터 크기가 바뀔 수 있는 데이터 타입은 heap에 올라가게 됨...


아래의 경우는 non-primitive인 string을 다루므로 string은 heap 메모리 영역에 할당됨..
main함수와 s1이라는 변수 자체는 stack 영역에 할당되고 s1은 heap memory의 pointer값을 갖고 있게 됨
s2로 ownership을 옮기게 되면...
s1의 pointer 자체를 삭제하고 s2에 s1의 pointer를 넘기게 됨
s3는 s2의 reference를 참조하여 s2의 메모리를 가리키는 pointer를 갖게 됨...
s4는 s2의 값을 복사해서 갖고 있게 되므로 또다른 heap에 s2와 같은 데이터를 갖는 또 다른 데이터를 새 heap 메모리 위치에 갖게 되고 s4는 그 위치를 가리키는 pointer를 갖게 됨
fn main()
{
    let x =5;
    let s1 = String::from("문자열 아무것");
    let s2 = s1; //move
    let s3 = &s2;
    let s4 = s2.clone(); //copy

}
 */

const MAX_VALUE:i32 = 40000;

fn main()
{
    let (x, y) = (2,4);
    let sum_value = square_sum(x,y);
    println!("합의 제곱 : {}", sum_value);
}

fn square_sum(num1:i32, num2:i32) ->i32
{
    let result  = square(num1+num2);
    result
}

fn square(num:i32) -> i32
{
    num*num
}

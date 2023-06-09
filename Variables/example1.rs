
fn main()
{
    //fn : 함수 앞에 붙이는 것
    //
    //주석 처리
    /*
    여러 줄 주석처리
     */
    ///<>
    /// 
    
    println!("Hello World");
    print!("한 줄 처리");
    println!("The value is {}", 100);
    print!("이것은 출력 커맨드입니다");
    print!("이것은 같은 줄에 이어서 문자열을 출력할 것입니다");
    print!("이것은 같은 줄에 이어서 문자열을 출력할 것입니다");
    print!("이것은 같은 줄에 이어서 문자열을 출력할 것입니다");
    print!("이것은 같은 줄에 이어서 문자열을 출력할 것입니다");
    print!("
    여러 줄에 
    걸쳐 
    문자가 출력될까?
    ");
    println!("\n\n이 줄은 두 줄 밑에 나타날 것");
    println!("\t이 줄은 앞에 빈 칸을 두고 나타날 것");
    ///----------
    /// 변수와 scalar data type
    /// let a = 5;라는 식으로 하면 컴파일러가 자동으로 i32라는 정수형태를 뒤에 붙여서 처리함
    /// -------
    let x = 15;
    println!("x의 값 : {}", x);
    //rust의 let 변수는 바꿀 수 없는 값이다.. > 그래서 나중에 값을 바꾸려고 시도하면 에러를 띄움
    //x = 5;
    //rust에서 쓸 수 있는 것 : 일반적인 변수는 지정된 형태에서 바뀌지 않는데.. 변수의 형태가 바뀔 수 있다면...?
    let mut a = 5;
    //원래는 데이터 타입이 바뀌려고 시도하면 에러를 띄우나 mut라는 것으로 이 값이 바뀔 수 있음을 컴파일러에 알려줌...
    a= 6;
    //변수 이름 짓기...
    //문자, 숫자, _만 가능
    //문자, _로 시작해야 함
    //대, 소문자 구분함 즉 Last, last는 다른 변수로 취급함

    //scalar type
    //integer : 정수>
    //+-있는 것과 없는 것이 있음
    //+-있으면 i8, i16, i32, i64
    //+-없으면 u8, u16, u32, u64

    let y = 20;
    println!("u8의 가장 큰 숫자 {}", std::i8::MAX);
    //float : f32, f64
    let z = 3.6;
    //boolean
    let status = false;
    println!("현재 갖고 있는 변수들의 값: {:?}", (x, y, z, status));

    let not_equals = 18 !=18;
    println!("조건 값 : {}", not_equals);
    //characters
    //
    let c1 = 'a';
    let c2 = '3';
    let c3 = '+';
    let c4 = '\u{288a}';
    let c5 = '\"';
    println!("변수들 값 {:?}", (c1,c2,c3,c4,c5));
}
// strings
//고정된 길이의 문자열 : &str
//길이가 바뀔 수 있는 문자열 : String

fn main()
{
    let some_string = "고정된 문자열";
    let mut growing_string = string::from("이 문자열은 길이를 바꿀 수 있습니다");
    println!("문자 확인 : \"{}\"", growing_string);
    growing_string.pop();
    println!("문자열 : \"{}\"", growing_string);
    growing_string.push_str(string: "내가 쓸 것");
    println!("문자열 : \"{}\"", growing_string);
    println!("
    문자열 기본 함수
    is_empty(): {},
    length(): {}.
    Bytes(): {},
    Contains use: {}
    ", growing_string.is_empty(), growing_string.len(), growing_string.capacity(), growing_string.Contains("내가"));
    growing_string.push_str(string: "       ");
    let str_len = growing_string.trim().len();

    let number = 6;
    let num_str = number.to_string();

    let some_char = 'a';
    let char_str = some_char.to_string();

    let my_name = "Noume".to_string();
    //아래와같이 하면 빈 문자열 0길이의 문자열이 하나 생성됨....
    let empty_string = String::new();

}
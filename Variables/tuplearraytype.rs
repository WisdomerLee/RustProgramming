//
// Tuple, Array
//

fn main()
{
    //묶음 형태로 된 데이터 형태 : Tuple
    let my_information = ("Salary", 4000);
    println!("{}는 이만큼 {}", my_information.0, my_information.1);
    println!("다른 Tuple 표현 방법 {:?}", my_information);

    let (mysalary, mysalary_value) = my_information;
    let mysalary = my_information.0;
    let mysalary_value = my_information.1;

    let nested_tuple = (4.5, (3,2), "문자");
    let element = nested_tuple.2.0;

    let empty_tuple = ();

    let mut number_array [i32;5] = [4,5,6,7,8];

    println!("{:?}", number_array);
    println!("{}", number_array[0]);

    number_array[4] = 5;
    //아래와같이 선언하면 0으로 초기화된 10개짜리 배열이 생성됨
    let array_with_same_elements = [0;10];

    let mut string_array=["사과", "포도", "배"];

    let string_array = ["Unknown";6];
    string_array[0] = "딸기";
    //0부터 3칸단위로 끊어서 배열을 갖고 옴..
    let subset_array = &number_array[0..3];
    let subset_array = &number_array[0..=3];

    println!("배열의 원소 : {}", number_array.len());
    println!("배열은 {} 바이트를 차지함", std::mem::size_of_val(&number_array));
}
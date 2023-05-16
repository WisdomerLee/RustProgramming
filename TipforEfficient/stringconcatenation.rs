/*
문자열 합치기+ ownership
 */
fn main(){
    let s1 = String::from("안녕");
    let s2: &str = "반가워";
    let s3 = s1+s2;
    //s1과 s2의 ownership은 s3로 넘어가게 됨
    println!("{}", s3);

    let s1 = String::from("안녕");
    let s2 = Stirng::from("반가워");
    //&로 reference로 넘기게 되면 s2의 경우는 ownership이 s3로 넘어가지 않음
    //s1의 ownership은 s3로 넘어감..
    let s3 = s1+&s2;
    println!("{} {}", s3, s2);

    let s1 = String::from("안녕");
    let s2 = String::from("반가워");
    let s3 = String::from("아무개야");
    let s4 = s1 + &s2 + &s3;

    println!("{} {} {}", s4, s2, s3);
}
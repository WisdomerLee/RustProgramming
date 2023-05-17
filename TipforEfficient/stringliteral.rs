/*
문자열 Literal
 */
fn main(){
    let str1 = "그 사람이 말했어요 \"안녕이라고\"";
    //위의 표현에서 역슬래시를 종종 사람이 빼먹을 수도 있고 하므로...
    let str2 = r#"그사람이 말했어요"안녕""#;
    //r#"출력하고 싶은 문자열"#사이에 두면... 문자열이 그대로 출력됨!!!
    let str3 = r"그 사람이 말했어요 \n \t '";
    //r""사이에 두면 \t, \n, \" 과같은 특수 기능을 하는 것들도 기능을 하지 않고 문자 그대로 출력됨

    let jason_str = r#"{
        "name" : "이아손",
        "age" : 40,
        "sex" : Male,
        "#;
    //문자열 중간에 #을 넣고 싶으면 r##""## << 맨 처음과 끝에 넣고 싶은 #보다 1개 더 많은 #을 붙이면..
    let str4 = r##"Hello"# world"##;
}
/*
정규 표현식 기본
Cargo.toml의 
[dependancies]
regex = "1.7.0"


 */

extern crate regex;
use regex::Regex;

fn main(){
    let re = Regex::new(r"[prt]ain").unwrap(); //이 패턴은 pain, rain, tain을 찾을 것
    //let re = Regex::new(r"[prt].ain").unwrap(); //이 패턴의 경우는? rrain을 찾게 됨...
    let text = "rrrain spain none";
    //is_match()함수는 있는지 없는지만 확인함
    println!("문자열에 해당 패턴 등이 있는가? {:?}", re.is_match(text));
    //겹치는 내용이 어디에 있는지를 찾고 싶다면 find()를 쓸 것
    println!("문자열에 있는 패턴 위치는 {:?}", re.find(text));

    for cap in re.captures_iter(text) {
        println!("맞는 패턴 {:?}", &cap[0]);
    }

    let re = Regex::new(r"gr[ae]y").unwrap();
    let text = "gray grey graye";

    for cap in re.captures_iter(text) {
        println!("맞는 패턴 {:?}", &cap[0]);
    }
    //a부터 z까지 aain~zain까지 찾음
    let re = Regex::new(r"[a-z]ain").unwrap();
    //let re = Regex::new(r"[A-Z]ain").unwrap();//이러면 대문자로 시작하는 ain시리즈를 찾음
    //만약 특정 문자를 제외한 형태를 찾고 싶다면...? :a~z로 시작하는 ain시리즈를 제외하고 ain시리즈를 찾음
    //let re = Regex::new(r"[^a-z]ain").unwrap();
    let text = "main pain tain rain 0ain";

    for cap in re.captures_iter(text) {
        println!("맞는 패턴 {:?}", &cap[0]);
    }
    //네자리 숫자 찾기 : \d\d\d\d :~자리 숫자를 찾으려면 \d를 그만큼 반복하면 됨..

    let re = Regex::new(r"\d\d\d\d").unwrap();
    //let re = Regex::new(r"\d....").unwrap();<<혹은 이렇게
    //물론 제외하려면
    //let re = Regex::new(r"^\d....").unwrap();
    let text = "내 폰은 8160, 두번째 폰은 8166";

    for cap in re.captures_iter(text){
        println!("맞는 패턴 {:?}", &cap[0]);
    }
    //아래의 경우는 포함하는 경우가 아닌 정확히 맞는 단어만 찾음
    let re = Regex::new(r"^aba").unwrap();
    let text = "aba abaa bc";

    for cap in re.captures_iter(text) {
        println!("맞는 패턴 {:?}", &cap[0]);
    }
    //아래의 경우는 bc뒤에 무엇이 붙은 경우에만 찾을 것 bc1, bca, bcd 등..
    let re = Regex::new(r"bc$").unwrap();
    let text = "aba abc bc";

    for cap in re.captures_iter(text) {
        println!("맞는 패턴 {:?}", &cap[0]);
    }
    //아래의 경우는 숫자 2개가 붙어있는 경우를 모두 찾게 됨
    let re = Regex::new(r"^\d\d$");

    //아래의 경우는 단어의 시작 스펠링을 찾게 됨...
    let re = Regex::new(r"\b\w").unwrap();
    //만약 뒤에 *를 붙이게 된다면 : 단어 단위로 찾게 됨
    //let re = Regex::new(r"\b\w*").unwrap();
    let text = "Hi my name is Princept";

    for cap in re.captures_iter(text) {
        println!("맞는 패턴 {:?}", &cap[0]);
    }

}
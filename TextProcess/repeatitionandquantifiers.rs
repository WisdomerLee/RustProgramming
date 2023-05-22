/*
Regexes : 반복, Quantifiers

Cargo.toml의 
[dependancies]
regex = "1.7.0"

 */

extern crate regex;
use regex::Regex;

fn main(){
    //?를 붙이면 a가 있어도 좋고 없어도 좋은 것이라 aa를 찾고 aaa도 찾음
    let re = Regex::new(r"a?aa").unwrap();
    let text = "aa aaa";
    
    for cap in re.captures_iter(text) {
        println!("패턴 {:?}", &cap[0]);
    }

    //아래의 경우는 b를 기본으로 a는 선택적으로 찾게 됨 : b, ba등을 찾게 됨
    let re = Regex::new(r"ba?").unwrap();
    let text = "a ba b ba";
    
    for cap in re.captures_iter(text) {
        println!("패턴 {:?}", &cap[0]);
    }
    //아래의 경우는 .rs중에서도 파일 이름이 세 글자 이하인 것들만 찾게 됨
    let re = Regex::new(r"\w?\w?\w?.rs").unwrap();
    let text = "fil.rs, t1,rs file.rs";
    
    for cap in re.captures_iter(text) {
        println!("패턴 {:?}", &cap[0]);
    }
    //아래의 경우는 a들로만 된 부분만 찾음
    let re = Regex::new(r"a+").unwrap();
    let text = "a aa aaa baab bab";
    
    for cap in re.captures_iter(text) {
        println!("패턴 {:?}", &cap[0]);
    }
    //아래의 경우는 .gif를 포함하는 단어를 찾게 됨
    let re = Regex::new(r"\w+\.gif").unwrap();
    let text = "image1.gif and background.gif";
    
    for cap in re.captures_iter(text) {
        println!("패턴 {:?}", &cap[0]);
    }
    //아래의 경우는 a뒤에 b가 0~여럿 반복되는 단어들을 찾음
    let re = Regex::new(r"ab*").unwrap();
    let text = "a ab abbbbb";
    
    for cap in re.captures_iter(text) {
        println!("패턴 {:?}", &cap[0]);
    }

    //3~5글자를 찾음 : 제한된 크기의 단어를 찾게 됨...여섯이상의 글자는 다섯글자만을 잘라 보여주게 됨
    let re = Regex::new(r"\w{3,5}").unwrap();
    let text = "hello i think you are happy because i have a gift for you";
    
    for cap in re.captures_iter(text) {
        println!("패턴 {:?}", &cap[0]);
    }
    //3~5글자를 찾음 : 제한된 크기의 단어를 찾게 됨... : 다만 온전한 단어만 찾음 바로 위의 경우와는 달리 becau부분이 사라지게 됨
    let re = Regex::new(r"\b\w{3,5}\b").unwrap();
    let text = "hello i think you are happy because i have a gift for you";
    
    for cap in re.captures_iter(text) {
        println!("패턴 {:?}", &cap[0]);
    }

    //숫자 1~3개 사이에 . 숫자 1~3개 사이의 숫자를 찾게 됨 1.1~ 111.111까지 찾음
    let re = Regex::new(r"\d{1,3}.\d{1,3}").unwrap();
    let text = "921.583 0.0 1456.25";
    
    for cap in re.captures_iter(text) {
        println!("패턴 {:?}", &cap[0]);
    }
    //숫자 1~3개 사이에 . 숫자 1~3개 사이의 숫자를 찾게 됨 1.1~ 111.111까지 찾음 : 숫자가 넘어가면 찾지 않음
    let re = Regex::new(r"\b\d{1,3}.\d{1,3}\b").unwrap();
    let text = "921.583 0.0 1456.25";
    
    for cap in re.captures_iter(text) {
        println!("패턴 {:?}", &cap[0]);
    }
    //숫자 3개 이상이 붙어있는 경우 모두 찾음
    let re = Regex::new(r"\d{3,}").unwrap();
    let text = "5321 500 5862342 25256 21 1 0 50";
    
    for cap in re.captures_iter(text) {
        println!("패턴 {:?}", &cap[0]);
    }
    //아래의 경우는 숫자 4개 - 숫자 2개 - 숫자 2개 의 패턴을 찾아 돌려줌
    let re = Regex::new(r"(\d{4})-(\d{2})=(\d{2})").unwrap();
    let text = "2012-03-05, 2017-05-05, 2023-05-05";
    
    for cap in re.captures_iter(text) {
        println!("{} 년 {} 월 {} 일 {}",&cap[1],&cap[2], &cap[3], &cap[0]);
    }
    


}
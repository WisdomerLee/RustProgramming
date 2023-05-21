/*
file handling

 */

use std::fs::*;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

fn basic_file_handling() -> std::io::Result<()> {
    let path_loc = r"d:\my_test.txt";
    let path = Path::new(path_loc);
    let mut file = File::creat(path)?;

    file.write(b"let's put this in the file")?;
    file.write("let's put this in the file".as_bytes())?;
    
    //append : 참으로 넣으면 파일을 덮어 씌우지 않고 기존 내용에 추가하는 형태가 됨
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write("\n이것저것 넣어봅시다".as_bytes())?;

    let str1 = "아무개";
    file.write(str1.as_bytes())?;

    //벡터를 저장하기
    let some_vec = vec![1,2,3,4,5,6];
    let str_from_vec = some_vec.into_iter().map(|a|format!("{} \n", a.to_string()))
    .collect::<String>();

    file.write(str_from_vec.as_bytes())?;

    //format이라는 매크로를 써 출력의 형태를 지정할 수도 있음
    let (name, age) = ("요셉", 40);
    let formatted_str = format!("난 {}, 나이는 {}", name, age);
    file.write(formatted_str.as_bytes())?;

    //파일 불러오기는 아래와 같이...
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("파일은 {:?}", contents);

    //파일을 불러오되 한줄씩 콘솔창에 띄우려면... : BufReader의 형태로 파일을 불러올 수도 있음..
    let mut file = File::open(path)?;
    let file_buffer = BufReader::new(file);
    for lines in file_buffer.lines(){
        println!("{:?}", lines?);
    }

    Ok(())
}

fn main(){
    basic_file_handling();
}
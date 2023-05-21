/*
디렉토리(폴더)와 경로 관련 함수들
 */

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main(){
    //파일 경로 한 번에 지정하기
    let path = Path::new(r"D:\Rust\Examples\my_file.txt");
    println!("폴더가 파일에 포함되어있나요: {:?}", path.parent().unwrap());

    println!("파일 이름은 {:?}", path.file_stem().unwrap());
    println!("파일의 확장자는 {:?}", path.extension().unwrap());


    //파일 경로 순차로 집어넣기
    let mut path = PathBuf::from(r"D:\");
    path.push(r"Rust");
    path.push(r"Examples");
    path.push(r"my_file");
    path.set_extension("txt");

    println!("파일 경로는 {:?}", path);
    //배열로 된 부분을 파일 경로로 재구성하기
    let path = [r"D:\", r"Rust", r"Examples", r"my_file.txt"]
    .iter().collect::<PathBuf>();
    println!("경로는 {:?}", path);
    //is_dir()로 폴더 경로를 확인할 수 있음
    let path = Path::new(r"D:\Pics");
    println!("폴더 경로가 맞는가? {:?}", path.is_dir());
    //파일이 존재하는지를 is_file()로 확인할 수 있음
    let path = Path::new(r"D:\my_text.txt");
    println!("이 파일이 존재하는가? {:?}", path.is_file());
    //Path에 있는 메타 데이터
    let data = path.metadata().unwrap();
    println!("파일의 타입 {:?}", data.file_type());
    println!("데이터 크기 {}", data.len());
    println!("권한 {:?}", data.permissions());
    println!("수정된 날짜 {:?}", data.modified());
    println!("생성된 날짜 {:?}", data.created());


    //폴더의 파일 모두 보여주기
    let path = Path::new(r"D:\");
    for files in path.read_dir().expect("폴더 읽기 실패"){
        println!("{:?}", file.unwrap().path());
    }
    //프로그램이 실행되는 현재 폴더 접근
    let mut curr_path = env::current_dir().expect("현재 폴더 접근 실패");
    println!("{:?}", curr_path);
    //폴더 만들기
    println!("폴더 만들기 {:?}", fs::create_dir(r"D:\Rust1"));
    //폴더 및 하위 폴더 동시에 만들기
    println!("폴더와 그 밑의 하위폴더 동시에 만들기 {:?}", fs::create_dir_all(r"D:\Rust1\level1\level2"));
    
    //폴더 및 파일 지우기 : 아래의 경우는 빈 폴더일 때만 가능.. 만약 해당 폴더에 하위 폴더 및 파일이 있으면 지우기에 실패함
    println!("특정 폴더 지우기 {:?}", fs::remove_dir(r"D:\Rust1\level1\level2"));
    //파일이 있는 폴더를 통째로 지우고 싶다면...?
    println!("해당 폴더의 모든 것을 지우기 {:?}", fs::remove_dir_all(r"D:\Rust1"));

    println!("파일 지우기 {:?}", fs::remove_file(r"D:\my_test.txt"));
    //파일 이름 바꾸기 : 파일의 전체 경로로 넣어야 함
    println!("파일 이름 바꾸기 {:?}", fs::rename(r"D:\prev.txt", r"D:\new.txt"));
    //복사하기
    println!("파일에서 파일 복사하기 {:?}", fs::copy(r"D:\new1.txt", r"D:\new2.txt"));
    
}
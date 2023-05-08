/*
모듈을 만들려면 일단 프로젝트를 만들어야 함
terminal에서 cargo new 프로젝트 이름 의 형태로 프로젝트를 만들고..

mod 모듈 이름 형태로 생성함..
 */

//아래와 같이 use를 이용하여 module의 함수를 호출하거나..

//use moduleproject::file_1;
//use moduleproject::file_2;

//mod : 사용할 mod이름
//외부 모듈 쓰기는
//crates.io쪽에서 검색해서 모듈을 찾고
//project에서 Cargo.toml쪽의 dependency쪽에 모듈 이름, 버전을 넣는다


use array_tool::vec::*;


fn main() {
    //moduleproject::file_1::printing();
    //moduleproject::file_2::printing();
    // let rect1 = Rectangle{
    //     length: 5,
    //     width: 10,
    // };

    // let area_rect1 = file_1::rect_area(&rect1.length, &rect1.width);
    
    let vec_1 = vec![1,1,3,5,7,9,11];
    let vec_2 = vec![3,5,6];

    let intersection = vec_1.intersect(vec_2.clone());
    println!("겹치는 것: {:?}", intersection);
    
    let union_set = vec_1.union(vec_2.clone());
    println!("합친 것: {:?}", union_set);

    println!("3번 보이기 {:?}", vec_2.times(3));
}

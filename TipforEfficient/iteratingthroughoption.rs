/*
    선택사항에 따라 반복
 */


fn main(){
    let some_product = Some("노트북");
    let mut product_vec = vec!["핸드폰", "배터리", "충전기"];

    //아래와 같은 형태로 접근하지 말고...
    // match some_product{
    //     Some(product) => product_vec.push(product),
    //     _=>{}
    // }

    // if let Some(product) = some_product{
    //     product_vec.push(product);
    // }
    //한줄로 처리할 수 있음....
    product_vec.extend(some_product);
}
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
    println!("{:?}", product_vec);
    
    //아래의 예시는 products_iter에 chain이라는 함수로 묶기, 각 iter들을 Some이라는 값으로 묶음
    let products_iter = product_vec.iter().chain(some_product.iter());
    
    for prod in products_iter{
        println!("{:?}", prod);
    }
    
    let products = vec![Some("Charger"), Some("Battery"), None, Some("Cellphone")];

    //아래와 같은 코드를....
    // let mut prod_without_none = Vec::new();
    // for p in products{
    //     if p.is_some(){
    //         prod_without_none.push(p.unwrap());
    //     }
    // }
    // println!("{:?}", prod_without_none);
    //아래와 같은 코드가 같은 결과를 얻어냄..
    let prod_without_none = products.into_iter().filter(|x| x.as_ref().is_some()).map(|x| x.unwrap())
    .collect::<Vec<&str>>();
    //아래의 경우는 products의 벡터를 none의 값을 없앤 벡터로 바꾸어줌...
    let products_flattened: Vec<&str> = products.into_iter().flatten().collect();

}
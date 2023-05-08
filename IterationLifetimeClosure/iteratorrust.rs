/*
반복..?
iterator
 */
fn main(){
    let some_vec = vec![2,3,56,65];
    let mut iter = some_vec.iter();

    println!("iterator: {:?}", iter);

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    let a = vec![0,3,5,8,11,20,24,35,88,95,102];

    let mut check = a.iter().any(|&x| x>0);
    println!("any함수의 값 {}", check);

    let mut check = a.iter().all(|&x| x>=0);
    println!("all 함수의 값 {}", check);
    let check = a.iter().find(|&&x| x>6);

    println!("check 함수의 값 {}", check.unwrap());

    let check = a.iter().position(|&x| x>4);
    println!("위치 함수의 값 {}",  check.unwrap());

    let check = a.iter().rposition(|&x| x>4);
    println!("오른쪽 위치 함수의 값 {}", check.unwrap());

    let check = a.iter().max();
    println!("최대 함수의 값 {}", check.unwrap());

    let check = a.iter().min();
    println!("최소 함수의 값 {}", check.unwrap());
    
    let mut iter = a.iter().rev();
    println!("뒤집기 함수의 값 {:?}", iter);
    println!("{:?}", iter.next());


    let a = vec![0,1,3,4,6,7,9,10];
    let filtered_values = a.iter().filter(|&x| *x>=5).collect::<Vec<&u32>>();
    println!("{:?}", filtered_values);

    let filtered_values = a.into_iter().filter(|&x| x>=5).collect::<Vec<u32>>();
    println!("{:?}", filtered_values);

    let b = a.clone();

    let mut mapped_values = b.iter().map(|x| 2 * * x).collect::<Vec<u32>>();
    println!("{:?}", mapped_values);
    let mut mapped_values = b.iter().map(|x| 2 * * x).filter(|x|*x>10).collect::<Vec<u32>>();
    println!("{:?}", mapped_values);

    //https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
}
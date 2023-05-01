/*
러스트의 ownership
함수에서는 어떤 방식으로 동작하는가??
 */

fn stack_function(mut var:i32){
    var = 56;
    println!("Var : {}", var);
}

fn heap_function(var: &mut Vec<i32>){
    var.push(50);
    println!("Var: {:?}", var);
}

fn main()
{
    let stack_num = 32;
    let mut heap_vec = vec![4,5,6];

    stack_function(stack_num);
    println!("메인의 stack_num : {}", stack_num);

    heap_function(&mut heap_vec);
    println!("메인의 heap_vec : {:?}", heap_vec);

    let som_vec = vec![4,5,6];
    let ref1 = som_vec;
    let ref2 = &ref1;

    //let mut vec1 = vec![4,5,6];
    //let mut ref1 = &vec1;

    let large_data1 = String::from("매우 긴 문자열");
    let large_data2 = String::from("또다른 매우 긴 문자열");

    let huge_data = vec![&large_data1, &large_data2];
}
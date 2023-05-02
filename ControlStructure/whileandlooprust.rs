/*
while and simple loop
 */

fn main()
{
    /*
    loop {
        println!("무한 반복 출력!");
    }
     */
    
    let mynumber =5;
    let mut guess = false;
    println!("1과 20 사이의 숫자를 예측해봅시다");
    
    while !guess{
        let mut number = String::new();
        std::io::stdin()
        .read_line(&mut number)
        .expect("읽기 실패");
        let number = number.trim().parse().expect("숫자가 아님");
        
        if mynumber == number{
            println!("맞았어요!");
            guess = true;

        } else{
            println!("다시 시도해보아요");
        }
    }

    println!("2와 5로 나누어지는 숫자를 하나 얻어봅시다");

    let mut number = String::new();
    std::io::stdin()
    .read_line(&mut number)
    .expect("숫자 형태가 아님");

    let mut number = number.trim().parse().expect("잘못된 숫자 형태");
    let mut dividable = false;

    while !(number%2==0 && number%5==0) {
        number = number+1;
    }
    println!("숫자가 2,5 둘다 나누어 떨어짐 {}", number);
    
    


}
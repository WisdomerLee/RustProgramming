/*
break and continue
 */
fn main()
{
    //break 반복문 종료
    //continue :반복문 중에 현재의 반복실행을 다음으로 넘김
    let mut var = 100;
    loop
    {
        var = var - 1;
        if var %13 == 0
        {
            break;
        }
    }
    println!("100보다 작은 13의 배수 중 가장 큰 수는 {}", var);

    let mut var = 0;
    let mut count = 0;
    let reqnumber = loop 
    {
        var= var + 1;
        if var %5 ==0 &&var %3 ==0
        {
            println!("3과 5의 공배수는 {}\n", var);
            count = count + 1;
            if count == 3
            {
                break;
            }
            else 
            {
                continue;
            }
        }
        println!("{}", var);
    };
    println!("3번째로 만족하는 숫자 {}", reqnumber);
    let mut x = String::new();
}
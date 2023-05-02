/*
control structure
conditional if and variables
 */

fn main()
{
    //간단한 if
    /*
        if condition {
            //condition이 참일 때 실행될 내용
        }
     */
    let some_number = 40;
    if some_number < 50 {
        println!("숫자가 50보다 작음");
    }
    println!("이건 조건과 무관하게 실행될 것");

    let marks = 65;
    if marks >=60 && marks<=70 {
        println!("값이 적정한 값에 들어감");
    }

    let flag1 = true;
    let flag2 = false;

    if flag1 ||flag2{
        println!("적어도 하나 이상의 조건을 충족함");
    }

    let flag1 = true;
    if flag1 != false{
        println!("flag1이 참 혹은 거짓이 아닐 경우 호출됨");
    }

    let flag1 = true;
    let flag2 = false;
    let number = 60;
    if(flag1 && flag2 == false) || number<50{
        println!("");
    }

    /*
    if-else condition

    if condition {
        //조건이 참일 때 실행될 내용
    } else {
        //조건이 거짓일 때 실행될 내용
    }
     */

    let marks = 80;
    if marks>90{
        println!("통과");
    } else {
        println!("실패...");
    }

    /*
    if-else if ...
    if condition{

    } else if condition1{

    }
    else if condition2{

    } else if condtion3{

    }
    ...
    else{

    }
     */

    let marks = 95;
    let mut grade = 'N';
    if marks >=90{
        grade = 'A';
    } else if marks >=80{
        grade = 'B';
    } else if marks >=70{
        grade = 'C';
    }
    /*
    nested if
     */
/*
    if outer_condition{
        if inner_condition{

        }else{

        }
    } else{

    }
 */

    println!("Enter a number");

    let mut some_num = String::new();

    std::io::stdin()
    .read_line(&mut some_num)
    .expect("입력 읽기에 실패");
    
    let some_num = some_num.trim().parse().expect("잘못된 값");

    if some_num != 0 {
        if some_num %2 ==0{
            println!("짝수");
        } else{
            println!("홀수");
        }
    } else{
        println!("0은 짝수도, 홀수도 아닙니다");
    }

    /*
    if let

    let variable num = if condition {
        //무언가 실행될 것
        //돌려주는 값 뒤에는 세미콜론 생략
    } else{
        //무언가 실행될 것
        //돌려주는 값 뒤에는 세미콜론 생략
    };
     */

    let value = if true{
        1
    } else{
        2
    };

    println!("값은 : {}", value);

    

}
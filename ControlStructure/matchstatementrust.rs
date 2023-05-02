/*
match statement
 */
fn main()
{
    /*
    match value{
        possible_value(s) => {statement},
        possible_value(s) => {statement},
        possible_value(s) => {statement},
        _ => {statement}//위의 조건들 중 하나의 조건도 맞지 않을 때 실행될 것
    }
     */

    let some_number = 100;
    match some_number{
        1 => println!("숫자는 1임"),
        2|3=>println!("2혹은 3"),
        4..=100 => println!("숫자는 4와 100사이"),
        _=> println!("100보다 큰 숫자")
    }
    
    let marks = 50;
    let mut grade = 'N';

    match marks{
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        70..=79 => grade = 'C',
        60..=69 => grade = 'D',
        _ => grade = 'F',
    }
    println!("성적은 {}", grade);
    /*
    let variable = match value{
        possible_value(s) => {statement},
        possible_value(s) => {statement},
        possible_value(s) => {statement},
        _ => {statement}//위의 조건들 중 하나의 조건도 맞지 않을 때 실행될 것
    }
     */
    let marks = 50;
    let mut grade = 
    match marks{
        90..=100 => {
            //여러 줄로 되어도 상관은 없으나 돌려주는 값 뒤에는 반드시 세미콜론이 빠져야 함...
            let mut b= 50;
            'A'},
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    };
    println!("성적은 {}", grade);

}
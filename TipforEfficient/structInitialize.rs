/*
struct 객체 초기화
 */
#[derive(Debug, Default)]
struct Student{
    id: u8,
    name: String,
    age: u8,
}
//객체의 속성을 무분별하게 노출하지 않으면서도 객체의 모든 값을 초기화 시키고 싶다면..
impl Student{
    // fn new(std_name: String) -> Self{
    //     Self{
    //         id: 0,
    //         name: std_name,
    //         age: 20,
    //     }
    // }

    fn new(stdname: String) -> Result<Self, String>{
        if std_name.chars().all(|x| matches!(x, 'a'..='z')){
            Ok(Self{
                id: 0,
                name: std_name,
                age: 20,
            })
        }
        else{
            Err("잘못된 이름".to_string())
        }
    }
}
/*
impl Default for Student{
    fn default() -> Self{
        Self{
            id: 0,
            name: "알려지지 않음".to_string(),
            age: 20,
        }
    }
}
 */


fn main(){
    //unwrap_or_default함수 : Ok, Err로 나온 것을 Ok일 땐 object로, Err일 땐 기본 값으로 적용
    //let new_student = Student::new("아무개".to_string()).unwrap_or_default();
    let new_student = Student::default();
    println!("{:?}", new_student);

    let new_student = Student{
        age: 12,
        //설정하지 않은 나머지 필드는 기본 값으로..
        ..Default::default()
    };
}
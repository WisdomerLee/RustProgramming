/*
Structure

 */
struct Person{
    citizenship : String,
    name: String,
    age: i32,
    gender: char,
    salary: i32
}
//structure implement : 같은 이름의 structure를 집어넣어 함수를 추가할 수 있음..
impl Person{
    //new라는 키워드로 만들게 되면 default값으로 자동 지정
    fn new()-> Self{
        Person { citizenship: String::from("미국"), name: String::from("잔"), age: (40), gender: ('M'), salary: (100) }
    }
    /*
    혹은 
    fn new() -> Person{
        Self{
            citizenship: String::from("미국"), name: String::from("잔"), age: (40), gender: ('M'), salary: (100)
        }
    }
     */
    fn compute_taxes(&self) -> f32{
        (self.salary as f32/3.) *0.5
    }
}


/*
Tuple Structure

 */
struct Numbers(i32, i32);

impl Numbers{
    fn greater(&self) ->i32{
        if self.0 > self.1 {
            self.0
        } else{
            self.1
        }
    }
    fn lesser(&self) -> i32{
        if self.0 < self.1 {
            self.0
        } else{
            self.1
        }
    }
}


fn main(){
    let person1 = Person{
        name: String::from("아무개"),
        citizenship: String::from("왈도국"),
        age: 40,
        gender: 'M',
        salary: 40000,
    };
    println!("사람 정보 : {} {} {}", person1.citizenship, person1.age, person1.gender);
    println!("세금 정보 : {}는 {}을 내야 합니다", person1.name, person1.compute_taxes());
    let person2 = Person::new();

    println!("기본 값으로 저장된 사람");
    println!("이름 {}, 시민권 {}", person2.name, person2.citizenship);

    let person3 = Person{
        age:70,
        name: String::from("사로"),
        ..person1 //처리하지 않은 나머지는 이미 생성된 person1의 데이터로 초기화 하겠다는 것

    };

    let mut person4 = Person::new();

    person4.name = String::from("개명");


    let some_num = Numbers(32, 16);
    println!("구조체가 가진 두 데이터 {} {}", some_num.0, some_num.1);
    

}
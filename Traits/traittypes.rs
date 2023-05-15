/*
Scope of Trait : trait은 선언된 모듈 내에서만 쓰일 수 있음, pub이 붙으면 외부 모듈에서도 쓸 수 있음
Trait methods with same name for a type
super trait
marker trait
auto trait
 */

trait Pilot{
    fn fly(&self);

}
trait Wizard{
    fn fly(&self);
}

struct Human;
impl Pilot for Human{
    fn fly(&self){
        println!("기장이 말합니다");
    }
}
impl Wizard for Human{
    fn fly(&self){
        println!("마법의 힘으로!");
    }
}
impl Human{
    fn fly(&self){
        println!("팔을 휘둘러도 날 수 없어!");
    }
}

//rust에서는 상속의 개념이 없음

trait Person{
    fn name(&self) -> &str;
}
//student trait에 person의 trait이 implement됨 : person이 super trait이 됨..
trait Student: Person{
    fn complete_info(&self) -> (&str, u8, &str);
}

struct uni_student{
    name_std: String,
    age: u8,
    university: String,
}
//student자체도 person을 implement된 것이므로 person도 implement시켜야 함..
impl Student for uni_student{
    fn complete_info(&self) -> (&str, u8, &str){
        (&self.name(), self.age, &self.university)
    }
}

impl Person for uni_student{
    fn name(&self) -> &str{
        &self.name_std
    }
}

fn info<S:Student>(s: &S){
    println!("{:?}", s.complete_info());
    println!("{:?}", s.name());
}

trait Programmer{
    fn fav_language(&self) -> String;
}
//두가지 이상의 trait을 impl programmer와 student가 compscistudent의 super trait이 됨
trait CompSciStudent: Programmer + Student{
    fn git_username(&self) -> String;
}
//marker trait
trait Some_properties: Clone+ PartialEq + Default{}
//#[derive(타입)]: 해당 타입의 trait을 모두 넣겠다는 것 : 일일이 impl로 하지 않아도 됨...
#[derive(Default, Clone, PartialEq)]
struct AcademyStudent{
    name: String,
    age: u8,
    nationality: String,
}
impl Some_properties for Student{

}
//auto trait
#[derive(Default)]
struct Customer{
    name: String,
    age: u8,
    relationship: Visit,
}

enum Visit{
    casual,
    new,
    frequent,
}

impl Default for Visit{
    fn default() -> Self{
        Self::new
    }
}


fn main{
    let person = Human;
    //기본적으로 fn fly가 3개가 human에 들어갔음
    //저렇게 되면 impl Human<< 이것이 불리게 됨(기본 함수 처럼)
    person.fly();
    //pilot의 fly를 호출하기
    Pilot::fly(&person);
    //wizard의 fly를 호출하기
    Wizard::fly(&person);

    let s = uni_student{
        name_std: "아무개".to_string(),
        age: 40,
        university: "핵".to_string(),
    };

    s.complete_info();
    s.name();

    info(&s);

    let c_1 = Customer::default();
    

}
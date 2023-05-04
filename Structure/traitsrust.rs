/*
Traits : C#의 interface와 비슷한 역할을 하는 것 같음....? 함수의 형태를 지정하고 실제 함수의 구현은 다른 곳에서 실현...
 */

struct Person{
    citizenship: String,
    name: String,
    age: u8,
    gender: char,
    salary: i32,
}

struct Student{
    name_std: String,
    age: u8,
    sex: char,
    country: String,
}

trait GeneralInfo{
    fn info(&self) -> (&str, u8, char);
    fn country_info(&self) -> str;
}
//해당 trait을 person에 적용
impl GeneralInfo for Person{
    fn info(&self) -> (&str, u8, char){
        (&self.name, self.age, self.gender)
    }
    fn country_info(&self) -> &str{
        &self.citizenship
    }
}

impl GeneralInfo for Student{
    fn info(&self) -> (&str, u8, char){
        (&self.name_std, self.age, self.sex)
    }
    fn country_info(&self)-> &str{
        &self.country
    }
}

struct Circle{
    radius: f32,
}

struct Rectangle{
    length: f32,
    width: f32,
}

trait ShapeInfo{
    fn area(&self){
        println!("trait 함수 내부에 기본 값을 갖고 있고 다른 structure등에 집어넣을 때 해당 함수를 정의하지 않으면 trait의 기본 함수가 실행됨...");
    }
    fn perimeter(&self);
}

impl ShapeInfo for Circle{
    fn area(&self){
        let area_of_circle = 3.14 * (self.radius *self.radius);
        println!("동그라미 넓이 : {}", area_of_circle);
    }
    fn perimeter(&self){
        let circumference = 2.0 * 3.14 * self.radius;
        println!("동그라미 둘레 : {}", circumference);
    }
}

impl ShapeInfo for Rectangle{
    fn area(&self){
        let area_of_rect = self.length * self.width;
        println!("사각형 넓이: {}", area_of_rect);
    }
    fn perimeter(&self){
        let perimeter_of_rect = 2.0 * (self.length + self.width);
        println!("사각형 둘레 : {}", perimeter_of_rect);
    }
}

fn main(){
    let person = Person{
        name: String::from("아무개"),
        citizenship: String::from("연"),
        age: 40,
        gender: 'M',
        salary: 4000,
    };
    let student = Student{
        name_std: String::from("학생 아무개"),
        age: 15,
        sex: 'M',
        country: String::from("연"),
    };

    let c1 = Circle{
        radius: 3.2,
    };
    let r1 = Rectangle{
        width: 5.0,
        length: 4.0,
    };
    c1.area();
    c1.perimeter();
    r1.area();
    r1.perimeter();

}
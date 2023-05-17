/*
to do macro, 또다른 유용한 처리 기법
todo!() : 아직 완성되지 않았으나 나중에 함수 혹은 나머지 내용을 처리하겠다는 것...
우선순위에 맞추어 다른 기능들은 todo!()로 두고 순서대로 처리하여도 됨.. : 대신 실행하면 panic이 실행됨...
todo plugin이 필요함...

 */
#[derive(Default)]
struct Student{
    name_std: String,
    age: u8,
    sex: char,
    country: String,
    salary: u32,
    nationality: String,
}

impl Student{
    fn some_fn_1(&self) -> String{
        todo!()
    }
    fn some_fn_2(&self) -> u8{
        todo!()
    }
}

trait GeneralInfo{
    fn info(&self) -> (&str, u8, char);
    fn country_info(&self) -> &str;
}

impl GeneralInfo for Student{
    fn info(&self) -> (&str, u8, char){
        todo!()
    }
    fn country_info(&self) -> &str{
        todo!()
    }
}

fn main(){
    let student_1 = Student::default();
    student_1.some_fn_1();
}
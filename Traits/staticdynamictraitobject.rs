/*
Static vs Dynamic

static이 속도도 빠르고 좋은 편 : generic type으로 만든 trait은 compile될 때 실제 쓰이는 타입에 대한 함수만 남기므로...

dynamic은 속도는 느린 편
dynamic은 코드는 줄일 수 있음
다만 실행할 때 object를 찾아 해당 함수를 찾고 검색하는 시간이 있으므로 실행에 시간이 걸리는 편
하지만 대응되는 타입의 갯수가 몹시 많을 경우.. dynamic을 활용할 것...
 */

trait Print{
    fn print(&self);
}

impl Print for String{
    fn print(&self){
        println!("값 {}", self);
    }
}
impl Print for i32{
    fn print(&self){
        println!("값 {}", self);
    }
}

fn display<T: Print>(x:T){
    x.print();
}


#[derive(Debug)]
struct Person{
    name: String,
}
struct Student{
    name: String,
}

trait info{
    fn info(&self);
}

impl info for Person{
    fn info(&self){
        println!("{:?}", self.name);
    }
}

fn static_dispatch<T: info> (t: &T){
    t.info();
}

fn dynamic_dispatch(t: &dyn info){
    t.info();
}


//dynamic object로 처리하기
fn display_dynamic(x: Vec<&dyn Print>){
    for i in x{
        i.print();
    }
}
//dynamic 을 box smart pointer로 처리하면..?
fn display_dynamic(x: Vec<Box<dyn Print>>){
    for i in x{
        i.print();
    }
}



fn main(){
    display(5);
    display("안녕".to_string())

    let p1 = Person{
        name: "아무개".to_string(),
    };

    let p2 = Student{
        name: "학생".to_string(),
    };
    static_dispatch(&p1);
    static_dispatch(&p2);
    dynamic_dispatch(&p1);
    dynamic_dispatch(&p2);
}
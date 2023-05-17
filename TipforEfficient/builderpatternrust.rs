/*
Builder Pattern
rust에서는 builder pattern이 다른 프로그래밍 언어보다 더 자주 보이는데 그 이유는 rust에서는 method overloading을 제공하지 않기 때문...
 */
#[derive(Debug, Default, Clone)]
struct Customer{
    name: String,
    username: String,
    membership: MembershiopType,
    gender: char,
    country: String,
    age: u8,
}

#[derive(Debug, Clone)]
enum MembershiopType{
    new,
    casual,
    loyal,
}

impl Defualt for MembershiopType{
    fn default() -> Self{
        MembershiopType::new
    }
}
//속성을 여럿 초기화 할 수 있는 상황이 오면.. 계속 new, new2, new3등이 계속 생겨날 수 밖에 없음....
impl Customer{
    // fn new(name: String) -> Self{
    //     Self { 
    //         name: name,
    //         ..Default::default()
    //      }
    // }
    // fn new2(name: String, username: String) -> Self{
    //     Self{
    //         name: name,
    //         username: username,
    //         ..Defualt::default()
    //     }
    // }
    // fn new3(name: String, username: String, membership: MembershiopType) -> Self{
    //     Self { name: name,
    //             username: username, membership: membership,
    //         ..Default::default()
    //      }
    // }
    fn new(name: String) -> CustomerBuilder{
        CustomerBuilder { name: name,
            username: None,
            membership: None,
            gender: None,
            country: None,
            age: None }
    }
}

//필수적인 필드 하나를 제외하고 Option<>으로 묶임
struct CustomerBuilder{
    name: String,
    username: Option<String>,
    membership: Option<MembershiopType>,
    gender: Option<char>,
    country: Option<String>,
    age: Option<u8>,

}
impl CustomerBuilder{
    //각 필드별로 속성을 변경할 수 있는 함수가 있고 그 함수는 각각 structure 자체를 돌려줌..
    fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    fn membership(&mut self, membership: MembershiopType) -> &mut Self{
        self.membership = Some(membership);
        self
    }

    fn gender(&mut self, gender: char) -> &mut Self{
        self.gender =  Some(gender);
        self
    }
    fn country(&mut self, country: String) -> &mut Self{
        self.country = Some(country);
        self
    }
    fn age(&mut self, age: u8) -> &mut Self{
        self.age = Some(age);
        self
    }
    //속성이 추가되면 해당 속성에 대응되는 함수만 이곳에 추가하면 됨... : new1, new2, new3로 넘어갈 필요가 없음..
    //마지막으로 build 함수로 원하는 객체를 실행함...: 입력한 것에 따라 객체를 생성할 수 있도록...없으면 default값으로 설정할 수 있도록 진행
    fn build(&mut self) -> Customer{
        Customer { 
            name: self.name.clone(), 
            username: self.username.clone().unwrap_or_default(), 
            membership: self.membership.clone().unwrap_or_default(), 
            gender: self.gender.unwrap_or_default(), 
            country: self.country.clone().unwrap_or_default(), 
            age: self.age.unwrap_or_default() }
    }
}



fn main(){
    let new_user = Customer::new("아무개".to_string()).build();

    let user_with_login = Customer::new("길동".to_string()).username("길동".to_string()).build();

    let user_with_membership = Customer::new("할".to_string()).username("압".to_string()).membership(MembershiopType::loyal).build();

}
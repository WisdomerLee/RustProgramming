/*
데이터를 처리하되 데이터가 처리된 후엔 데이터는 보여지기만 하고 수정되면 안 될 경우..
데이터는 처음에 설정된 이후에 값이 변경되면 안된다는 것...
 */
#[derive(Debug, Clone)]
struct finalized_config<T>(T);

impl <T> Copy for finalized_config<T> where T: Copy{

}
impl <T> std::ops::Deref for finalized_config<T>{
    type Target = T;
    fn deref(&self) -> &T{
        &self.0
    }
}
#[derive(Debug)]
struct Config{
    a: usize,
    b: String,
}
impl Config{
    fn new() -> self{
        Self{a:0, b:String::from("Hello")}
    }

    fn build(self) -> finalized_config<Config>{
        finalized_config(self)
    }
}

fn main(){
    //아래와 같이 data가 정렬되기 전의 형태로 받아서
    let mut data = vec![5,6,8,3,4,5];
    //정렬이 된 이후
    data.sort();
    //그 뒤엔 데이터의 값이 바뀌면 안 되는 형태여서... 바꿀 수 없는 변수 형태로 두는 것
    let data = data;
    //위의 예시는 중간의 정렬되기 전, 혹은 그 직후에 데이터에 추가 수정이 가해질 수 있다는 것(개발자가 원하지 않는 데이터 변화가 있을 수 있음)
    //

    let data = {
        let mut data = vec![5,4,6,8,2,1];
        data.sort();
        data
    };
    //바로 위의 예시는 데이터가 처음 선언된 이후엔 변경할 수 없는 타입으로 바뀐다는 점에서 바로 직전의 예시보단 낫지만....
    //초기화 시키는 과정이 매우 길 경우엔 관리하기가 어려움..>> 다른 코드들과 분리시키기가 쉽지 않은 형태
    //초기화 시키는 과정을 별도의 모듈로 분리시켜...그 모듈 내에서 초기화를 처리하도록 하기...

    let mut my_configuration = Config::new();
    my_configuration.a = 6;

    let finalized = my_configuration.build();

    //아래와 같이 복사해서 처리하려고 해도 데이터를 변형시킬 수 없게 됨
    // let mut finalized_copy = finalized;
    // finalized_copy.a = 60;

}
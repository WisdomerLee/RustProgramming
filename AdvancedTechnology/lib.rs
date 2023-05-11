#[cfg(test)]
mod tests{
    #[test]
    fn it_works(){
        assert_eq!(12,12);

    }
    //함수 위에 #[test]를 붙여야만 testing이 시작됨.. assert_함수들을 이용하여 원하는 값이 제대로 나오는지 확인을 할 때 쓰임
    //#[test]
    fn test2(){
        assert_eq!(true, false);
    }
}

struct Circle{
    radius : f32,
}

impl Circle{
    fn area(&self) -> f32{
        3.14 * (self.radius * self.radius)
    }
    fn perimeter(&self) -> f32{
        2.0 *3.14 *self.radius
    }
    fn contains(&self, other: &Circle) -> bool {
        self.radius > other.radius
    }
}

#[cfg(test)]
mod test2{
    use super::*;
    #[test]
    fn larger_circle_should_contain_smaller(){
        let larger_circle = Circle{
            radius: 5.0,
        };
        let smaller_circle = Circle{
            radius: 2.0
        };

        assert!(larger_circle.contains(&smaller_circle));
    }
    #[test]
    fn smaller_circle_should_not_contain_smaller(){
        let larger_circle = Circle{
            radius: 5.0,
        };
        let smaller_circle = Circle{
            radius: 2.0
        };

        assert!(!smaller_circle.contains(&larger_circle));
    }
    #[test]
    fn larger_circle_should_have_larger_area(){
        let larger_circle = Circle{
            radius: 5.0,
        };
        let smaller_circle = Circle{
            radius: 2.0
        };
        assert!(larger_circle.area()>smaller_circle.area());
    }
}

fn division(divident: f64, divisor: f64) -> Option<f64> {
    match divisor {
        0.0 => None,
        _=> Some(divident/divisor),
    }
}

#[cfg(test)]
mod testdivision{
    use super::*;

    #[test]
    fn test(){
        let divident = 10.0;
        let divisor = 5.0;
        let result = division(divident, divisor);

        assert!(result != None);
    }
}

struct Person{
    name: String,
    salary: i32,
}
impl Person{
    fn salary_range(&self){
        panic!("연봉이 범위에서 벗어났습니다!");
    }
}

#[cfg(test)]
mod testdivision2{
    use super::*;
    //아래의 경우는 테스트할 때 panic이 일어나야 함을 알려주고 하기 때문에 panic이 일어나지 않으면 test가 실패하게 됨....
    #[test]
    #[should_panic]
    fn out_of_range(){
        let some_person = Person{
            name: String::from("아무개"),
            salary: 65000,
        };
        some_person.salary_range();
    }
}
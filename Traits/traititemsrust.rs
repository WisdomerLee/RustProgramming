/*
Trait Items
Self,
Functions and Methods
Generic Parameters and Associated Types
 */

trait Simple_trait{
    fn fn_1() -> i32;
    //Trait의 Self는 trait이 import된 객체 자체를 돌려줌...?
    fn fn_2() -> Self;
}
//unit structure: field가 하나도 없는 structure
struct SomeType;
struct OtherType;

impl Simple_trait for SomeType{
    fn fn_1() -> i32{
        5
    }

    fn fn_2() -> Self{
        SomeType
    }
}

impl Simple_trait for OtherType{
    fn fn_1() -> i32{
        6
    }
    fn fn_2() -> Self{
        OtherType
    }
}

trait Default{
    fn default() -> Self;
}

impl Default for i32{
    fn default() -> Self{
        0
    }
}

trait Trait_methods{
    //첫번째 함수에선 trait이 들어가는 것 자체의 타입으로 입력이 들어가고
    fn fn_1(Self);
    //두번째 함수에선 자체의 reference의 타입이 들어감
    fn fn_2(&self);
    //세번째 함수에선 변경 가능한 reference 타입으로 들어감
    fn fn_3(&mut self);
    //self: 함수가 불리는 객체 자체
    //Self: 함수가 불리는 것의 타입!!이라는 차이가 있음
}

trait Add{
    //Associate type 설정
    type Rhs;
    type Output;
    fn add(self, rhs: Self::RHs) -> Self::Output;
}
//Generic type으로 변경 : Generic type으로 바꾸면 같은 trait을 type에 따라 여러 번 implement할 수 있음
trait Add<Rhs>{
    type Output;
    fn add(self, rhs:Rhs) -> Self::Output;
}
//Generic type으로 변경
trait Add<Rhs, Output>{
    fn add(self, rhs:Rhs) -> Output;
}
struct Point{
    x: i32,
    y: i32,
}

impl Add for Point{
    type Rhs = Point;
    type Output = Point;
    fn add(self, rhs: Self::Rhs) -> Self::Output{
        Point{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for Point{
    type Rhs = i32;
    type Output = Point;

    fn add(self, rhs: i32) -> Point{
        Point{
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Add<Point> for Point{
    type Output = Point;
    fn add(self, rhs:Point) -> Point{
        Point{
            x: self.x+rhs.x,
            y: self.y+rhs.y,
        }
    }
}

impl Add<i32> for Point{
    type Output = Point;
    fn add(self, rhs: i32) -> Point{
        Point{
            x: self.x+rhs,
            y: self.y+rhs,
        }
    }
}

impl Add<Point, Point> for Point {
    fn add(self, rhs: Point) -> Point{
        Point{
            x: self.x+ rhs.x,
            y: self.y+ rhs.y,
        }
    }
}
impl Add<i32, Point> for Point{
    fn add(self, rhs:i32) -> Point{
        Point{
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

struct Line{
    start: Point,
    end: Point,
}

impl Add<Point, Line> for Point{
    fn add(self, rhs: Point) -> Line{
        Line{
            start: self,
            end: rhs,
        }
    }
}
fn main(){
    let zero:i32 = Default::default();
    let i: i32;
    //i.default();
}
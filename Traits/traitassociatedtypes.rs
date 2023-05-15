/*
Associated types

시속과 거리의 관계는 속력에 시간을 곱하면 나온다. 둘 다 숫자형태의 데이터 형태로 나옴..
아니면 단위만 다른 mile같은 경우는...?
 */
#[derive(Debug)]
struct Kmh{
    value: u32,
}
#[derive(Debug)]
struct Km{
    value: u32,
}
#[derive(Debug)]
struct Mph{
    value: u32,
}
#[derive(Debug)]
struct Miles{
    value: u32,
}

//impl되는 형태가 kmh, mph의 논리가 같고 돌려주는 타입만 다름..
/*
impl Kmh{
    fn distance_in_three_hours(&self) -> Km{
        Km{
            value: self.value*3,
        }
    }
}

impl Mph{
    fn distance_in_three_hours(&self) -> Miles{
        Miles{
            value: self.value*3,
        }
    }
}

 */

trait DistanceThreeHours{
    //associate type
    type Distance;
    fn distance_in_three_hours(&self) -> Self::Distance;
}

impl DistanceThreeHours for Kmh{
    type Distance = Km;
    fn distance_in_three_hours(&self) -> Self::Distance{
        Self::Distance{
            value: self.value*3
        }
    }
}
impl DistanceThreeHours for Mph{
    type Distance = Miles;
    fn distance_in_three_hours(&self) -> Self::Distance{
        Self::Distance{
            value: self.value*3
        }
    }
}

fn main(){
    let speed = Kmh{ value: 90 };
    let distance = speed.distance_in_three_hours();
    println!("{}속도로 3시간 간 거리는 {}", speed, distance);

    let speed_mph = Mph{value: 90};
    let distance_miles = speed_mph.distance_in_three_hours();
    println!("{}속도로 3시간 간 거리는 {}", speed_mph, distance_miles);
}
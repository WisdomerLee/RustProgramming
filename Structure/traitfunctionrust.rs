/*
Trait - Function
 */

struct Data{
    some_data: Vec<i32>,
}

trait BasicStats{
    fn mean(&self) -> f32;
    fn variance(&self) ->f32;
}

impl BasicStats for Data{
    fn mean(&self) ->32{
        let mut sum = 0;
        for i in self.some_data.iter(){
            sum+= *i;
        }
        sum as f32 / self.some_data.len() as f32
    }
    fn variance(&self) -> f32{
        let mu = self.mean();
        let mut sum_squared_diff = 0.0;
        for i in self.some_data.iter(){
            sum_squared_diff += (*i as f32 -mu) * (*i as f32 -mu);
        }
        sum_squared_diff/self.some_data.len() as f32
    }
}
fn main(){
    let my_data = Data{
        some_data: vec![5,6,7,8,2,3,5];
    };
    println!("평균: {}", my_data.mean());
    println!("분산: {}", my_data.variance());
    
}
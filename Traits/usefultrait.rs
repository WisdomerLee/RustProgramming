/*
유용한 trait
 */
use std::cmp::Ordering;
#[derive(Clone)]
struct Person{
    name: String,
    age: u32,
    earning: u32,
    saving: u32,
}

impl PartialEq for Person{
    fn eq(&self, other: &Self) -> bool{
        self.age == other.age
    }
}

impl PartialOrd for Person{
    fn partial_cmp(&self, other:&Self) -> Option<std::cmp::Ordering>{
        self.earning.partial_cmp(&other.earning)
    }
}

fn main(){
    let bob = Person{
        name:"Bob".to_owned(),
        age: 30,
        earning: 30_000,
        saving: 50_000,
    };

    let mut bob_clone = bob.clone();
    bob_clone.age = 15;
    println!("{}", bob == bob_clone);
    println!("{}", bob >= bob_clone);
    println!("{}", bob <= bob_clone);
}
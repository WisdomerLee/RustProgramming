/*
Product Popularity using HashMap

hashmap, loops, conditional if
 */
use std::collections::HashMap;

fn popularity_analysis(scores: Vec<i32>) -> bool{
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..scores.len()-1 {
        if scores[i] > scores[i+1]{
            increasing = false;
        }
        if scores[i] < scores[i+1]{
            decreasing = false;
        }
    }
    return increasing||decreasing;
}

fn main(){

    let mut product = HashMap::new();

    product.insert("1번", vec![1,2,34,5]);
    product.insert("2번", vec![1,5,6,2,6,5,2]);
    product.insert("3번", vec![8,8,8,5,6,5,6,5]);

    for (product_id, popularity) in product{
        if popularity_analysis(popularity){
            println!("{} popularity is increasing or decreasing", product_id);
        }
        else{
            println!("{} popularity is fluctuating", product_id);
        }
    }
}
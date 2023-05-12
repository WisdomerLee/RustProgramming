/*
아이템 추천

Hashset, vector를 이용하기

hashset에서의 검색은 array에서의 검색보다 빠름...
 */

use std::collections::HashSet;

sn product_suggestions(product_prices:Vec<i32>, amount: i32) -> Vec<Vec<i32>>{
    let mut prices_hash = HashSet::new();
    let mut offers = Vec::new();
    
    for i in product_prices{
        let diff = amount -i;
        if prices_hash(&diff).is_none() {
            prices_hash.insert(i);
        } else{
            offers.push(vec![i, diff]);
        }

    }
    offers
}

fn main(){
    let product = vec![11,3,55, 34,54,35,1,34,5,234,52];
}
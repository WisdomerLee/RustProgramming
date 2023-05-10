/*
Repeating patterns

 */

macro_rules! string_concat{
    // ()=>{
    //     String::new()
    // };
    // ($some_str: expr) =>{{
    //     let mut temp_str = String::new();
    //     temp_str.push_str($some_str);
    //     temp_str
    // }
    // };
    // ($some_s1: expr, $some_s2:expr) =>{{
    //     let mut temp_str = String::new();
    //     temp_str.push_str($some_s1);
    //     temp_str.push_str($some_s2);
    //     temp_str
    // }
        
    // };
    //반복 패턴을 쓰지 않으면 위처럼 파라미터를 늘려갈 때마다 저렇게 하나씩 늘려가면서 해야 하지만....
    //반복패턴을 쓰게 되면 아래의 표기로 모두 가능하게 됨...
    //또한 반복 패턴은 반드시 뒤에 ,를 붙여야 함...
    ($($some_str:expr,) *) =>{{
        let mut temp_str = String::new();
        //$()표현은 반복하겠다는 것을 뜻함...
        $(temp_str.push_str($some_str);)*
        temp_str
    }
    };
}
macro_rules! vec_mac{
    ($($element:expr,) *)=>{{
        let mut some_vec = Vec::new();
        $(some_vec.push($element);)*
        some_vec
    }

    };
}

fn main(){
    let str_null = string_concat!();
    let str_single = string_concat!("First",);
    let str_dougle = string_concat!("First", "Second",);
    
    let string_vec = vec_mac!("ㅁ", "ㅅㄴ",);
}
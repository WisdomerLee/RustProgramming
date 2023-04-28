//Vectors

fn main()
{
    let mut number_vec: Vec<i32> = vec![4,5,6,7,9,193,2,5,2,3,556,7,10];

    println!("{}", number_vec[0]);
    println!("{:?}", number_vec);

    number_vec[4] = 5;
    println!("{:?}", number_vec);

    let array_with_same_elements = vec![0;10];

    let mut string_array = vec!["사과", "딸기", "배"];
    let string_array2 = vec!["Unknown";6];

    string_array[0] = "자두";

    let char_vec = vec!['a', 'p', 'p', 'l', 'e'];

    let subset_vec = &&number_vec[0..3];
    println!("원소들 {:?}", subset_vec);

    println!("원소들 숫자 {}", number_vec.len());

        let check_index = number_vec.get(100);
        println!("{:?}", check_index);
        number_vec.push(30);
        number_vec.push(40);
        println!("값 : {:?}", number_vec);

        number_vec.remove(5);
        println!("벡터 값 : {:?}", number_vec);
        println!("10이 벡터 값에 존재하나? {}", number_vec.contains(&10));


}
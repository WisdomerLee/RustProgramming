/*
for loop, 
 */
fn main()
{
    let mut some_vec = vec![45,30,85,90,41,39];
    //0..=5 : 0~5까지
    for i in 0..=5
    {
        println!("{}번째 벡터 원소는 {}", i, some_vec[i]);
    }

    //vector의 원소들을 차례대로 출력하기 : 값을 바꿀 순 없음
    for i in some_vec
    {
        println!("{}", i);
    }
    //vector.iter()를 쓰게 되면 데이터를 빌려올 수 있음(ownership 변화 없이) : 값을 바꾸려면 이렇게 하거나..
    for i in some_vec.iter()
    {
        println!("{}", i);
    }
    //혹은 아래와 같이 &를 붙여 애초에 벡터 자체를 빌려오게 되면.... iter() 함수를 호출한 것과 같은 효과를 얻을 수 있음
    for i in &some_vec
    {
        println!("{}", i);
    }
    //혹은 변경할 수 있는 reference를 얻으려면...
    for i in some_vec.iter_mut()
    {
        //변경할 수 있는 reference의 pointer의 데이터에 접근하려면 *를 붙여서 접근할 수 있음...
        //reference 자체는 pointer의 주소를 가리키므로 ..
        *i +=5;
        println!("{}", i);
    }
    //혹은 아래와 같이 변경할 수 있음
    for i in &mut some_vec
    {
        *i += 5;
        println!("{}", i);
    }

    println!("{:?}", some_vec);

}
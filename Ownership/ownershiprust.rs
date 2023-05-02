/*
Rust Ownership
3개의 큰 룰이 있음

러스트의 변수들은 그들을 호출하는 owner가 있음
한 번에 오직 1개의 owner를 가질 수 있음
owner가 범위에서 벗어나면 값도 사라짐



primitive : integer, float, byte, array,  : 크기 고정, 비어있는 값을 가질 수 없음
non-primitive : vector, string, 등.. : 크기가 바뀔 수 있음, 비어있는 값을 가질 수 있음

rust에서는 non-primitive 값들에 대해서 =은 copy가 아닌 move로 적용함..
아래의 예시에서 그와 같은 형태를 볼 수 있음
 */

 fn main()
 {
     let x = 32.2;
     let y = x; //primitive type이므로 copy가 실행되어 아래의 상황은 아무 문제 없이 실행됨
 
     println!("x : {}, y: {}",x,y);
 
     let s1 = String::from("abc");
     //string은 non-primitive 타입이므로 =은 copy가 아닌 move로 적용됨...
     //s2로 s1의 데이터의 ownership이 옮겨짐..
     //let s2 = s1;
     //아래와 같이 표시하면 move가 아닌, s1의 reference를 가리키게 됨..., ownership은 변함없이 s1이 갖고 있음 : 데이터를 빌려온다고 생각하면 된다고 함
     let s2 = &s1;
     //s2로 s1이 갖고 있던 데이터 abc의 ownership이 옮겨졌으므로 s1을 호출하려 하면 에러가 발생함... > 한 번에 1개의 ownership을 가지기 때문...
 
     println!("s1: {}, s2: {}", s1, s2);
 
     let vec1 = vec![5,6,7,8,9];
     //역시 마찬가지로 string과 같이 vector도 크기가 바뀔 수 있고 empty를 가질 수 있는 non-primitive type의 변수이므로...
 
     //let vec2 = vec1; //이렇게 표시하면 move가 되므로... 데이터의 ownership이 vec1에서 vec2로 옮겨지게 됨
     //아래와 같이 표기해야 reference의 데이터를 빌려오게 됨
     //let vec2 = &vec1;
     //혹은 복사의 형태로 들고오려면.. 아래와 같이 ...
     let vec2 = vec1.clone();
     println!("Vec1: {:?}, Vec2: {:?}", vec1, vec2);
 
 
     {
         let myname = String::from("내 이름은 ");
 
     }
     //이렇게 하면 myname은 이미 내부에서 정의되고 {}를 벗어나게 되면 해당 myname 변수는 메모리에서 버려지게 됨... 
     //해당 변수에 접근하려면 반드시 같은 {}내부에서 적용되거나 {}외부에서 {}내부를 참조해야 함
     //println!("내 이름은 : {}", myname);
 }
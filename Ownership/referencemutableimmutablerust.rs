/*
Reference Rule

하나의 수정 가능한 reference가 scope에 있음
많은 수정 불가능한 reference가 있음
수정 가능, 수정 불가능한 것은 동시에 존재할 수 없음
reference의 scope
scope내에서 수정 불가능한 reference는 데이터를 수정하면 안 됨
 */

fn main()
{
    let mut heap_num = vec![4,5,6];
    let ref1 = &mut heap_num;
    //이미 위에서 수정 가능한 reference를 선언하였으므로 아래의 수정 가능한 reference는 에러를 일으킴...
    let ref2 = &mut heap_num;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

    let mut heap_num = vec![4,5,6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    //ref1, ref2 둘 다 reference를 들고 오지만 immutable의 상태이므로 에러 없이 그대로 값을 들고 옴...
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

    let mut heap_num = vec![1,2,3];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    //위에서 이미 수정 불가능한 reference를 빌려왔으므로 동시에 수정 가능한 reference를 가져올 수 없어 에러를 일으킴... : 수정 가능, 수정 불가능한 reference는 동시에 존재할 수 없으므로..
    let ref3 = &mut heap_num;
    println!("ref1: {:?}, ref2: {:?}, ref3: {:?}", ref1, ref2, ref3);

    let mut heap_num = vec![3,4,5];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
    //이미 수정 불가능한 reference가 println!함수에서 호출되었으므로... 수정 가능한 reference를 선언할 수 있음
    //즉 처음 reference가 선언되고 나서 호출되고 나면 그 뒤에 수정 불가, 혹은 수정 가능한 reference를 선언할 수 있음..
    let ref3 = &mut heap_num;


    let mut heap_num = vec![4,5,6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    //수정 불가능한 reference가 선언되고 이 reference들이 호출되기 전에 변수를 수정하려 시도하기 때문에 컴파일러에서 에러를 띄움... 
    //하지만 호출되는 곳이 한 곳도 없게 되면...? 에러를 띄우지 않게 됨...
    //heap_num.push(60);
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
    //이미 수정 불가능한 reference가 함수에서 호출되었으므로 ....
    //아무런 문제 없이 데이터를 수정할 수 있게 됨...
    heap_num.push(50);

}
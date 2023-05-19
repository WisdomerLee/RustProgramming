/*
Async Await
//Cargo.toml파일에서 
[dependencies]
tokio = {version = "1.17", features = ["full"]}
tasks, select
task : thread보다 더 간단히 ..., blocking 없이 쓸 수 있음
 */

use tokio::select;

async fn printing(){

    println!("async 함수");
}

async fn huge_computation(s: String){
    println!("{:?} 시작",  s);
    for i in 0..1000000{

    }
    println!("{:?}  끝",  s);
}
async fn simpler_computation(s: String){
    println!("{:?} 시작",  s);
    println!("{:?}  끝",  s);
}

async fn function_1(){
    println!("첫번째 시작");
    for i in 0..10000000{

    }
    println!("첫번째 끝");
}

async fn function_2(){
    println!("두번째 시작");
    println!("두번째 끝");
}

#[tokio::main]
async fn main(){
    // let x = async 
    // {
    //     printing().await;
    // }.await; //일반 fn main()일 때
    let x = printing();
    println!("이것은 아직 ");
    //drop(x);//아직 실행되기 전에 실행을 취소시킬 수도 있음 실행을 뒤로 미루는 상태이기 때문..
    x.await;

    let mut handles = vec![];
    println!("이 부분은 async 부분이 아님");
    let s1 = String::from("시간 소요 함수 실행");
    let s2 = String::from("간단한 계산 함수 실행");

    let aw1 = tokio::spawn(async move{
        huge_computation(s1).await;
    });
    handles.push(aw1);

    let aw2 = tokio::spawn(async move{
        simpler_computation(s2).await;
    });
    handles.push(s2);
    
    for handle in handles{
        handle.await.unwrap();
    }
    println!("모든 태스트 완료");
    //하나가 완료되면 다른 태스크 실행이 끝남!!! : select
    //select는 비동기로 실행되는 여러 태스크들 가운데 가장 먼저 실행되는 것을 먼저 고르게 되고 그러면 나머지 작업들을 취소하게 됨..> 그런데 태스크 여럿을 병렬로 동시에 실행시키면 그 순서는 우리가 알 수 없음...> 실행할 때마다 랜덤하게 얻게 됨
    //
    tokio::select!{
        _ = function_1() => println!("첫번재 함수가 먼저 끝남"),
        _ = function_2() => println!("두번째 함수가 먼저 끝남"),
    };
    //함수 2개를 비동기로 대기시킨 상태로 결과를 얻게까지 동작한 다음 선택을 시키면....?
    let aw1 = tokio::spawn(async move{
        function_1().await;
    });
    let aw2 = tokio::spawn(async move{
        function_2().await;
    });
    //가장 먼저 동작이 끝난 것이 선택됨
    tokio::select!{
        _ = aw1 => println!("첫번째 함수 고름"),
        _ = aw2 => println!("두번째 함수 고름"),
    }
    //tokio의 join 매크로를 써보자...:: 그러면 함수가 순차적으로 실행, 완료되게 됨....
    tokio::join!{
        function_1(),
        function_2(),
    };
}
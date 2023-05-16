/*
struct 간단히 하기
 */

// struct A{
//     f1 : u32,
//     f2 : u32,
//     f3 : u32,
// }
// fn fn1(a: &mut A) -> &u32{&a.f2}

// fn f2(a: &mut A) -> u32{a.f1+a.f3}
// fn f3(a: &mut A){
//     let x = fn1(a);
//     let y = fn2(a);
//     println!("{}", x);
// }
//일반적으로 rust에서 struct의 모든 필드는 수정가능한 형태를 가짐, 또한 일반적으로 structure를 빌리게 되면 structure의 모든 부분을 통째로 빌려오게 된다
//structure의 각각의 필드를 서로 독립적으로 빌려올 수 있을까??
//mutable borrow는 한 번만, immutable borrow는 여러번 빌릴 수 있고 동시에 mutable, immutable borrow는 불가능하므로....

//structure를 보다 작은 structure로 쪼개어 만들어두면....?
struct A{
    b:B,
    c:C,
}

struct B{
    f2: u32
}
struct C{
    f1: u32,
    f3: u32,
}

fn fn1(b: &mut B) -> &u32{&b.f2}
fn fn2(c:&mut C) -> u32{c.f1 + c.f3}
fn fn3(a: &mut A){
    let x = fn1(&mut a.b);
    let y = fn2(&mut a.c);
    println!("{}", x);
}

fn main(){
    
}
//rust documentation에 들어가려면 "///"이것이 앞에 있어야 함..

//아래 내용은 rust doc에 들어가는 글귀가 됨...
//!# 기본
//! 
/// 입력 값을 숫자 계산
/// 
/// # 예시
/// # 테스트
/// ```
/// let n = 5;
/// let answer = new_project::square(n);
/// assert_eq!(25, answer);
/// ```
/// # 한계
/// 
/// # 다른 섹션
/// 
/// command 창에 cargo doc --open
//바로 윗줄까지 doc에 들어감
pub fn square(num: i32) -> i32{
    num*num
}
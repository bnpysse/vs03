// Gentle Intro Rust 第二部分 2023年1月21日0时29分6秒
// 结构 枚举 和匹配
// Rust 与其他语言有所不同,其它语言的变量总是会 引用 {references} 
fn main() {
    let s1 = "hello dolly".to_string();
    let s2 = s1;
    println!("s1 is {}", s1);
}

// 値を2倍にするジェネリクス関数
// 同じ変数を2度繰り返すことになるので、Copyトレイトを実装する
fn x2 <T: std::ops::Add<Output=T> + Copy> (n: T) -> T {
    n + n
}
fn main() {
    println!("{}", x2(3));
    println!("{}", x2(3.0f64));
    println!("{}", x2::<u64>(3));
}
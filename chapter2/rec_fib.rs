// フィボナッチ数を求める関数
fn fib(n:i64) -> i64 {
    if n == 1 { return 0; } //再帰条件の終了
    if n == 2 { return 1; } 
    return fib(n-2) + fib(n-1); //再起的に呼び出す
}

fn main() {
    for i in 2..=20 {
        println!("{}", fib(i));
    }
}
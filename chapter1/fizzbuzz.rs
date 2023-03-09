// rustでFizzBuzz問題を解く
fn main() {
    // 1から100まで繰り返す
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzuBuzz");
        } else if i % 3 == 0 {
            println!("Fizzu");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            // その数字iをそのまま表示
            println!("{}", i)
        }
    }
}
fn main() {
    // 1から7のうち奇数の値のみを出力する
    for i in 1..=7 {
        if i % 2 == 1 {
            println!("{}", i);
        }
    }
}
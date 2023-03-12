fn main() {
    // ブロック
    {
        let s1 = String::from("聞かないで返事をするのは愚か");
        println!("{}", s1);
    }
    // ブロックを抜けるとs1の値は破棄される
    // println!("{}", s1);
}
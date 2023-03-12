fn main() {
    // 文字列を生成
    let s = String::from("beep");
    // スライスを生成
    let ss = &s[0..3];
    // スライスの内容を表示
    println!("{}", ss);
}
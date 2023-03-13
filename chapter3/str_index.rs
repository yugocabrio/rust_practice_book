fn main() {
    let s = "こんばんは";
    // 先頭の1文字を取り出して表示
    let ch = s.chars().nth(0).unwrap();
    println!("{}", ch);
    let ch = s.chars().nth(2).unwrap();
    println!("{}", ch);
}
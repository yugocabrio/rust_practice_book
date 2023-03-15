fn main() {
    // 文字列を代入
    let array = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Mango".to_string(),
        "Tomato".to_string()
    ];
    // 配列をくり返し画面に表示する
    // 所有権は移動しない
    for a in array.iter() {
        println!("{}", a);
    }
    println!("len={}", array.len());
}
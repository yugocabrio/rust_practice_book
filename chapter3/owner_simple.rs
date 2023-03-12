fn main() {
    // Stringオブジェクトを生成して、変数g1に代入
    // Stringオブジェクトの所有者はg1です。
    // g1がStringオブジェクトの所有権を持っています。
    let g1 = String::from("穏やかな心は体に良い");
    // 所有権をg2に移動
    // オブジェクトの所有権はg1からg2に移動
    let g2 = g1; 
    println!("{}", g2);
}
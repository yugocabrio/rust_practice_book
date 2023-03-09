fn main() {
    // 値を２倍にするクロージャーを定義
    let x2 = |n| n * 2;
    // x2を使って見る
    println!("{}", x2(2));
    println!("{}", x2(8));
}
// 書き換え可能な変数の場合、let mut 変数名 = 値;
fn main() {
    let mut a = 100; //変更可能な変数aを定義
    a = a + 1; // 変数aの変更が可能
    println!("a is {}", a)
}
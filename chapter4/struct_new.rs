// 構造体personの定義
struct Person {
    name: String,
    age: i32,
}
// Personのメソッドを定義
impl Person {
    // Personを生成する関数を定義
    fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }
}
fn main() {
    // 関連関数を使ってオブジェクトを生成
    let taro = Person::new("太郎".to_string(), 18);
    // フィールドを確認
    println!("{}さんは{}歳", taro.name, taro.age);
}
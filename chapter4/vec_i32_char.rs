fn main() {
    // i32のVecを使う
    let mut v1: Vec<i32> = Vec::<i32>::new();
    v1.push(10);
    v1.push(20);
    v1.push(30);
    v1.pop(); // 末尾の要素を取り出す
    // 要素をくりかえす
    for i in v1.iter() {
        println!("{}", i);
    }
    // char型を使う
    let mut v2:Vec<char> = Vec::<char>::new();
    v2.push('a'); // 要素を末尾に追加
    v2.push('b');
    v2.push('c');
    v2.pop();// 末尾の要素を取り出す
    // 要素を繰り返す
    for i in v2.iter() {
        println!("{}", i);
    }
}
fn main() {
    // コマンドライン引数をVec<String>で得る
    // ベクター（可変配列）です！
    let args:Vec<String> = std::env::args().collect();
    println!("{:?}", args);
}
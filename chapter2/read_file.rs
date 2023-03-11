use std::env; // コマンドライン引数
use std::fs;  // ファイルの読み込みのため
fn main() {
    // 引数をベクターとして得る
    let args: Vec<String> = env::args().collect();
    // ファイル名の指定があるかどうか調べる
    if args.len() < 2 {
        println!("入力ファイルを指定してください");
        return;
    }

    // argsベクターの一番目の要素を得る
    // 0番目のこいつを動かすコマンド自身だから
    let filename = &args[1];
    // ファイルを読んで表示
    let text = fs::read_to_string(filename).unwrap();
    println!("{}", text);
}
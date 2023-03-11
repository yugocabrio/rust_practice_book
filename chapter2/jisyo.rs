use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // 辞書ファイルの指定
    let dicfile = "ejdict-hand-utf8.txt";
    // コマンドライン引数をベクターに入れる
    let args :Vec<String> = std::env::args().collect();
    // 引数のチェック
    if args.len() < 2 {
        println!("[USEAGE] jisyo ward");
        return;
    }
    // 指定された単語
    let word = &args[1];

    // ファルを開く
    let fp = File::open(dicfile).unwrap();
    // BufReaderで1行ずつ読む
    let reader = BufReader::new(fp);
    for line in reader.lines() {
        // 実際に1行取り出す
        let line = line.unwrap();
        // 行に単語が含まれているか検索
        if line.find(word) == None { continue; }
        println!("{}", line);
    }
}
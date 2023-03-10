use std::collections::HashMap;

fn main() {
    // 旧暦月名の一覧
    let tuki = ["睦月","如月","弥生","卯月","皐月","水無月","文月","葉月","長月","神無月","霜月","師走"];

    // HashMapを初期化
    // usizeは整数型
    let mut tuki_map: HashMap<&str, usize> = HashMap::new();
    // 月名をHashMapに追加
    // 配列.iter().emurate()と書くことで、（要素番号、値）のタプルを返すイテレーターが得られます
    // HashMapでキーと値のペアが表示される順番がシャッフルされてしまうのは、
    // ハッシュマップが要素の追加順ではなく、ハッシュ値に基づいて要素を格納するからです
    // 実行ごとにハッシュ値のランダム性で、順番が変わる
    for (i, v) in tuki.iter().enumerate() {
        tuki_map.insert(v, i+1);
    }
    println!("{:?}", tuki_map);
    // 要素を選んで表示
    println!("水無月 = {}月", tuki_map["水無月"]);
    println!("神無月 = {}月", tuki_map["神無月"]);
    println!("師走 = {}月", tuki_map["師走"]);
}
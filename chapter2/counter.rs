// rustで人気投票の集計
// HashMapを使うのに必要な宣言
// stdで始まるのはrustの標準ライブラリ
use std::collections::HashMap;
// 投票データを定数として宣言
// &strは文字列リテラルが定数（不変）である場合に使います。可変の場合はString型です。
const V_DATA: &str = "C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C,C";

fn main() {
    // 集計用のHashMapを生成（Pythonで言う辞書型です）
    // データを集計する用途に使うため、変数の宣言でmutをつけて可変であることを明示
    let mut c_map = HashMap::new();
    // HashMapに要素を追加して、0で初期化
    c_map.insert("A", 0);
    c_map.insert("B", 0);
    c_map.insert("C", 0);
    println!("{:?}", c_map);
    // 投票データを集計
    for w in V_DATA.split(',') {
        c_map.insert(w, c_map[w]+1);
        // insertメソッドを使ってキーとバリューを更新する
    }
    // 集計して結果を表示
    for k in ["A","B","C"] {
        println!("{}: {:>2}", k, c_map[k]);
        // {:>2}はフィールド2桁右寄せして表示できます
    }
}
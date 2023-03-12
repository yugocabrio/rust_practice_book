fn main() {
    // タプルを作る（&str, i64）
    let banana = ("バナナ", 300);
    let apple = ("りんご", 200);
    // タプルを参照して合計金額を求める
    let total = banana.1 + apple.1;
    // タプルの内容表示
    // 所有権がprint_tupleになるとダメなので、借用になるように&変数名にする
    print_tuple(&banana);
    print_tuple(&apple);
    println!("合計金額{}円です", total); 
}
// （商品名, 金額)のタプルを表示する関数
// タプルの参照&でっす！
fn print_tuple(item: &(&str, i64)) {
    println!("{}を{}円で購入", item.0 ,item.1);
}
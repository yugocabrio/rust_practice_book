// 配列をシャッフルするのに必要な宣言
use rand::seq::SliceRandom;

fn main() {
    // 1から75までの数値を配列に代入
    // 初期値0の75個の要素
    let mut nums = [0; 75]; 
    println!("{:?}", nums); // {:?}は、Rustのフォーマット文字列

    //各ループでnums配列のi-1番目の要素にiを代入します。
    for i in 1..=75 { nums[i-1] = i; }
    println!("{:?}", nums);

    // シャッフル
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng); // 引数の値をmutな参照として指定する
    println!("{:?}", nums);

    // カードの表示
    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;
            if i == 12 { //ワイルドカード
                print!(" *,");
            } else {
                print!("{:2},", nums[i]); // i番目の値を2桁で表す
            }
        }
        println!("");
    }
}

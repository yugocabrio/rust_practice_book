// BMIと肥満度を表示する関数
fn print_bmi(height: f32, weight: Option<f32>) {
    // weight に値があればBMIを求めて、Option型で返す
    let bmi:Option<f32> = match weight {
        Some(w) => Some(w / (height / 100.0).powf(2.0)),
        None => None,
    };
    // BMIの値に応じて判定
    let msg = match bmi {
        Some(n) if n < 18.5 => "低体重",
        Some(n) if n < 25.0 => "普通体重",
        Some(n) if n < 30.0 => "肥満1度",
        Some(n) if n < 35.0 => "肥満2度",
        Some(n) if n < 40.0 => "肥満3度",
        Some(_) => "肥満4度",
        None => "判定不能",
    };
    // 判定結果を入力を与える
    println!("BMI={:.1}, 判定{}", bmi.unwrap_or(0.0), msg);
    /*
    unwrap_or()は、Option<T>型の値から、その値がSomeの場合は中身の値を取り出し、Noneの場合は引数で指定したデフォルト値を返すメソッドです。
    具体的には、print_bmi()関数内で、bmi変数がSomeの場合はその値を取り出し、Noneの場合は0.0を返すようになっています。
    */
}
fn main() {
    let height = 16.3;
    print_bmi(height, Some(48.0));
    print_bmi(height, Some(72.3));
    print_bmi(height, None);
}
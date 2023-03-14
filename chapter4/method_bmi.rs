// 身長と体重を表すBody構造体
struct Body {
    height: f64, // 身長cm
    weight: f64, // 体重kg
}
// Body構造体のメソッドを定義
impl Body {
    // BMIを計算するメソッド
    fn calc_bmi(&self) -> f64 {
        let h = self.height / 100.0;
        // BMIの計算をして値を返す
        self.weight / h.powf(2.0)
    }
    // 乖離率を計算するメソッド
    fn calc_per(&self) -> f64 {
        self.calc_bmi() / 22.0 * 100.0
    }
}
// Body 構造体を触る
fn main() {
    let taro = Body{ height: 160.0, weight:70.0 };
    println!("BMI={:.2}", taro.calc_bmi());
    println!("乖離率={:.1}%", taro.calc_per());
}
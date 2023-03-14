// BMI判定
fn main() {
    let body = Body::new(163.0, 75.2, "田中");
    body.print_result();
    let body = Body::new(158.2, 55.0, "鈴木");
    body.print_result();
    let body = Body::new(174.2, 54.2, "井上");
    body.print_result();
}
// b判定用の構造体
struct BmiRange {
    min: f64,
    max: f64,
    label: String,
}
impl BmiRange {
    // オブジェクトを生成するメソッド
    fn new(min: f64, max: f64, label: &str) -> Self {
        BmiRange{ min, max, label: label.to_string() }
    } 
    // 範囲内かテストする関数
    fn test(&self, v: f64) -> bool {
        (self.min <= v) && (v < self.max)
    }
}
// 身長と体重を表す構造体
struct Body {
    height: f64, //cm
    weight: f64, //kg
    name: String,
}
impl Body{
    // オブジェクトを生成して返す
    fn new(height: f64, weight: f64, name: &str) -> Self {
        Body{ height, weight, name: name.to_string() }
    }
    // BMIを求める
    fn calc_bmi(&self) -> f64 {
        self.weight / (self.height / 100.0).powf(2.0)
    }
    // 肥満度判定を表示する
    fn print_result(&self) {
        // BMIを求める
        let bmi = self.calc_bmi();
        // 判定用のオブジェクトを配列で生成
        let bmi_lsit = [
            BmiRange::new(0.0, 18.5, "低体重"),
            BmiRange::new(18.5, 25.0, "普通体重"),
            BmiRange::new(25.0, 30.0, "肥満1度"),
            BmiRange::new(30.0, 35.0, "肥満2度"),
            BmiRange::new(35.0, 40.0, "肥満3度"),
            BmiRange::new(40.0, 99.9, "肥満4度"),
        ];
        let mut result = String::from("不明");
        // 配列を1つずつテストする
        for range in bmi_lsit {
            if range.test(bmi) {
                result = range.label.clone();
                break;
            }
        }
        println!("{}さん, BMI={:.1}, 判定={}", self.name, bmi, result);
    }
}
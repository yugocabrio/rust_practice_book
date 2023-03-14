// 宝箱の振る舞いを定義するトレイト
trait TreasureBox {
    // デフォルトメソッド
    fn open(&self, key_no: i32) -> bool {
        // 自身のキー番号とキー番号が一致すれば開く
        self.get_key_no() == key_no
    }
    fn check(&self);
    fn get_key_no(&self) -> i32;
}
// 宝石箱を定義
struct JewelryBox {
    price: i32,
    key_no: i32,
}
impl TreasureBox for JewelryBox {
    fn check(&self) {
        println!("宝石箱だった！金貨{}枚入手", self.price );
    }
    fn get_key_no(&self) -> i32 { self.key_no }
}
// 空箱を定義
struct EmptyBox {
    key_no: i32,
}
impl TreasureBox for EmptyBox {
    fn check(&self) {
        println!("空箱だった");
    }
    fn get_key_no(&self) -> i32 { self.key_no }
}
// 冒険者が箱を開ける
fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if !tbox.open(key_no) {
        // falseの時
        println!("鍵が合わず宝箱が開きません");
        return;
    }
    tbox.check();
}

fn main() {
    // 宝箱を用意
    let box1 = JewelryBox { price: 30, key_no: 1} ;
    let box2 = EmptyBox { key_no:1 };
    let box3 = JewelryBox { price: 50, key_no: 2 };
    // 箱を開ける
    open_box(&box1, 1);
    open_box(&box2, 1);
    open_box(&box3, 1);
}
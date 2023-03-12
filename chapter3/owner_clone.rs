fn main() {
    // ヒープ領域に割り当てられるStringも.cloneを使えば別の変数に値を複製することができる
    let g1 = String::from("穏やかな身体は体に良い");
    let g2 = g1.clone();
    println!("{}", g1);
    println!("{}", g2); 
}
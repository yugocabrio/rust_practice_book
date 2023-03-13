fn main() {
    let pr = "猫に小判";
    // 1バイトずつ表示
    for c in pr.bytes() {
        print!("{:2x}", c);
    }
    // バイト数を得る
    println!("\nバイト数={}B", pr.len());
}
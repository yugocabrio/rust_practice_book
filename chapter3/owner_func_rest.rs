fn main() {
    // g1の値が戻ってくるというのは、可変ということです。よってmut
    let mut g1 = String::from("過ちを見過ごす人は美しい");
    g1 = show_message(g1);
    println!("{}", g1); //ok
}

// stringを受け取り、Stringを返す関数
fn show_message(message: String) -> String {
    println!("{}", message);
    return message;
}
// メッセージを生成する関数
fn gen_message() -> String {
    let msg = String::from("過ちを見過ごす人は美しい");
    return msg;
}

fn main() {
    let m = gen_message(); // 所有権はmに移動
    println!("{}", m); //ok
}
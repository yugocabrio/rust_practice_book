// 引数の文字列を変更する関数
fn add_quote(msg: &mut String) {
    msg.insert(0, '『');
    msg.push('』');
}

fn main() {
    let mut msg = String::from("強い心は病気の支えとなる");
    println!("{}", msg);
    add_quote(&mut msg);
    println!("{}", msg);
}
// add_quote 関数は、与えられた文字列を変更するだけで、
// 新しい値を返す必要はありません。このため、add_quote 関数には return 文を必要としません。
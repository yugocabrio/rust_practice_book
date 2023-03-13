// &'static strのみ指定できる関数
fn echo(s: &'static str) {
    println!("{}", s);
}
fn main() {
    // 文字列リテラル(&'statice str)を指定
    echo("愚かな人でも黙っていると");
    echo("賢いと見られる");
}
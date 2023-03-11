use std::env;

fn main() {
    // argsはイテレータです。
    let args = env::args();
    println!("{:?}", args);
    for arg in args {
        println!("{}", arg);
    }
}
fn main() {
    // 先頭の文字を得る
    let s2 = "abcdefg";
    println!("{}", &s2[0..1]);
    // 先頭の1文字を取り出して表示
    let s = "こんにちは";
    let ch = &s[..3];
    println!("{}", ch);
    // （0から数えて）2文字目を取り出して表示
    let ch = &s[6..9];
    println!("{}", ch); // に

}
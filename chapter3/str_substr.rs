fn main() {
    let pr = "知恵は武器よりも価値がある";
    // 先頭の2文字を得る
    let mut sub1 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if i < 2 { sub1.push(c); continue; }
        break;
    }
    println!("先頭の2文字: {}", sub1);
    // 武器の部分を得る
    let mut sub2 = String::new();
    for (i, c) in pr.chars().enumerate() {
        // ０から考えて3から4文字目
        if 3 <= i && i <= 4 { sub2.push(c); } 
    }
    println!("4-5文字目: {}", sub2);
}
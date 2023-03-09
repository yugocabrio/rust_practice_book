fn main() {
    for i in 1926..2027 {
        // 西暦を改行なしで表示
        print!("西暦{}年", i);
        // 和暦を判定して改行つきで出力
        if i >= 2019 {
            if i == 2019 {
                println!("令和元年");
            } else {
                println!("令和{}年", i-2019+1);
            }
        } else if i >= 1989 {
            if i == 1989 {
                println!("平成元年");
            } else {
                println!("平成{}年", i-1989+1);
            }
        } else if i >= 1926 {
            if i == 1926 {
                println!("昭和元年");
            } else {
                println!("昭和{}年", i-1926+1);
            }
        }
    }
}
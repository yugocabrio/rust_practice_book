// mainの後の()を忘れない
fn main() {
    for i in 1..51 {
        if i % 3 == 0 {
            println!("A");
        } else if i % 10 ==3 {
            println!("A");
        } else {
            // "{}"を間違えていた
            println!("{}", i);
        }
    }
}
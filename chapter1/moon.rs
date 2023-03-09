fn main() {
    // rustは型に厳しいので整数と実数を明確に区別する必要がある
    let moon = 384400.0;
    let car = 80.0;
    let btrain = 300.0;
    println!("車で月まで{}日", moon / car / 24.0);
    println!("新幹線で月まで{}日", moon / btrain / 24.0);
}
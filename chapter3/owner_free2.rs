fn main() {
    // ブロック１
    {
        let s1 = String::from("真実はワインの中にある");
        let s3 = String::from("ブドウ畑と美人には手がかかる");
        // ブロック2
        {
            let s2 = s1;
            println!("{}", s2);
        }
        // s2の値はここで破棄される
        println!("{}", s3);
    }
    // s3の値はここで破棄される
}
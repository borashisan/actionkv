fn parity_bit(bytes: &[u8]) -> u8 {
    let mut n_ones: u32 = 0;

    for byte in bytes {
        let ones = byte.count_ones();
    }
}

fn main() {
    let abc = b"abc";
    println!("入力: {:?}", abc);
    println!("出力: {:08x}", parity_bit(abc));
    println!();
    let abcd = b"abcd";
    println!("入力: {:?}", abcd);
    println!("結果: {:08x}", parity_bit(abcd));
}

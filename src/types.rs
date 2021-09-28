pub fn run() {
    //int32
    let x = 1;

    //f64
    let y = 4.223;

    //explicit int
    let z: i64 = 65466546;

    //max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //boolean
    let a: bool = true;
    let b = (75 > 100);

    //char
    let c1 = '\u{1F620}';
    println!("{:?}", (x, y, z, a, b, c1));
}

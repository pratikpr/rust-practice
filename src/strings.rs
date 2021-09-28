pub fn run() {
    let s = "Hello";
    let mut s_m = String::from("World!");
    println!("{} and {}", s_m.len(), s_m.capacity());
    s_m.push('F');
    s_m.push_str("rom ther other side");
    println!("{:?}", (s, s_m));
}

pub fn run() {
    // Print to console
    println!("Hello Print World!");

    //Basic formatting
    println!("{} is {}", "Tierney", "Scottish");

    //named args
    println!("{name} is {nat} and {name} likes to {act}", name = "Tierney", nat = "Scottish", act = "Cross");

    // placeholder debug
    println!("{:?}", (12, true, "hello"));
    
}
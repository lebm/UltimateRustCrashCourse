fn main() {
    println!("Hello, world!");
    println!("{}", do_stuff(10.0, 20.0));
}

// Use snake case for function names
// Can appear anywhere
// parameters must have type annotations
// The last expression without ";" is a return value: { return 1; } is equal to { 1 }
fn do_stuff(qty: f64, oz: f64) -> f64 {
    qty * oz
}

use hello::greet;

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    println!("Hello, world!");

    greet();

    // Variables
    // Multiple assignments using destruct
    // Immutable by default: safety, concurrency and speed.
    let (_bunnies, _carrots) = (8, 50);

    // Error: see rustc --explain E0384
    // let x = 1;
    let mut x = 1;
    x = 2;

    // Constant
    // upper case, type annotation required, LHS must resolve at compile time.
    const WARP_FACTOR: f64 = 9.9;

    // Scope
    let y = 5;
    {
        let z = 99;
        println!("{y} {z}");
        // z is dropped at the end of the block.
    }
    // Error: z does not exist in this scope. rustc --explain E0425
    // println!("{y} {z}");

    // Shadowing
    let k = 1;
    {
        let k = 99;
        println!("{k}");
    }
    println!("{k}");

    // Shadowing in the same scope.
    // Useful for transformations to keep the same logical name but change the type.
    let val = "2";
    let val = val.parse::<i32>().unwrap();
    let val2 = "2.0";
    let val2 = val2.parse::<f64>().unwrap();

    // vars must be initialized.
    // This does not compile. enigma was not initialized. rustc --explain E0381
    // It is a undefined behavior
    // let enigma: i32;
    // println!("{enigma}");
}

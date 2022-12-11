fn main() {
    println!("Hello, world!");

    // 3 Rules
    // 1) Each value has an owner (a variable)
    // 2) Each value has only one owner
    // 3) When the owner goes out of scope, the values is dropped
    //
    // References (borrows) does not break these rules. References must respect the owners.
    // There must be only one reference (borrow or owner) allowed to modify a value.
    // When there is a mutable reference, no other reference can exist.
    let s1 = String::from("abc");
    let s2 = s1;
    // Wrong; s1 was moved to s2
    //print!(s1);
    println!("{s2}");
}

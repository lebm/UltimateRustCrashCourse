fn main() {
    // closure:
    // |x, y| { x // y }
    let s = "\u{1f353}".to_string();
    let f = || {
        println!("{}", s);
    };
    f();
}

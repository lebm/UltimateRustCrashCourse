#[allow(unreachable_code)]
fn main() {
    let num = 1;
    // if is expression
    // All blocks must retun the same type
    // {} area mandatory
    // There is no ternary operator. Use "if" instead.
    let _msg = if num == 1 { "one" } else { "two" };

    // "infinity" loop
    'bob: loop {
        loop {
            break 'bob;
        }
        continue;
    }
    let mut i = 10;

    // While loop ...
    while i > 0 {
        println!("{i}");
        i -= 1;
    }

    // for ..
    for num in [7, 8, 9].iter() {
        println!("{num}");
    }

    for (x, y) in [(1, 2), (3, 4)].iter() {
        println!("{x} {y}");
    }

    for num in 0..50 {
        println!("{num}");
    }
}

// enum is like ADT in Haskell
// enum values are called variants
// enum type name is the namespace of the variants: Color::Red
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

// variantes can have values, like "C" union
#[derive(Debug)]
#[allow(dead_code)]
enum DispenserItem {
    Empty,
    Ammo(u8),
    Things(String, i32),
    Place { x: i32, y: i32 },
}

use std::fs::File;

fn main() {
    let r = Color::Red;
    let g = Color::Green;
    let b = Color::Blue;
    println!("{r:?} {g:?} {b:?}");

    // Its is not idiomatic! Don't do this
    use DispenserItem::*;
    let item1 = Empty;
    let item2 = Ammo(50);
    let item3 = Things("Hat".to_string(), 5);
    let item4 = Place { x: 10, y: 20 };
    println!("{item1:?} {item2:?} {item3:?} {item4:?}");

    // Result enum
    let res = File::open("foo");
    // let _f = res.unwrap();
    // let _f = res.expect("foo not found");
    // if res.is_ok() {
    //     let _f = res.unwrap();
    // } else {
    //     println!("File not found error");
    // }
    match res {
        Ok(_) => println!("File exists"),
        Err(e) => println!("Erro {}. Foo does not exist", e),
    }
}

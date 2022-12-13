use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    v.push(6);
    let x = v.pop().unwrap();
    println!("{} {}", x, v[1]);

    // Type annotation is optional. It is inferred from the values pushed into the vec.
    // int values default to i32
    let mut v = Vec::new();
    v.push(3);
    v.push(6);
    v.push(9);
    let x = v.pop().unwrap();
    println!("{} {}", x, v[1]);

    // vec macro is a convenience
    let mut v = vec![4, 8, 12];
    let x = v.pop().unwrap();
    println!("{} {}", x, v[1]);

    // HashMap
    let mut h: HashMap<u8, bool> = HashMap::new();
    h.insert(5, true);
    h.insert(6, false);
    let have_five = h.remove(&5).unwrap();
    println!("{:?} {}", h, have_five);
}

#[allow(unused_variables)]
fn main() {
    // str => string slice. Almost never used directly. Can not be modified.
    // &str => borrowed string slice, or just string slice. It is what is used in practice. Can not be modified.
    // "literal string" => Is always (borrowed) string slice. Can not be modified.
    // String => heap allocated string type. Can be modified.
    // String => Metadata: len, capacity and reference to data on heap (&str)
    // Both are UTF-8. They can not be indexed like an array (UTF-8 restriction, grapheme)
    // There are lots of methods to manipulate String.
    let str_word = "A string (&str)";
    let string_word = "Another string (&str)".to_string();
}

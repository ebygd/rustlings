// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!



fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

// comments show initial score of r/w
fn main() {
    string_slice("blue");                                               // right
    string("red".to_string());                                          // right
    string(String::from("hi"));                                         // right
    string("rust is fun!".to_owned());                                  // WRONG
    string_slice("nice weather".into());                                // right
    string(format!("Interpolation {}", "Station"));                     // WRONG
    string_slice(&String::from("abc")[0..1]);                           // right
    string_slice("  hello there ".trim());                              // right
    string("Happy Monday!".to_string().replace("Mon", "Tues"));         // right
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());                    // WRONG
}

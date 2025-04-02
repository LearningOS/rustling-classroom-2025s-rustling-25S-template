// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");                         // "blue" 是字面量 → &str
    string("red".to_string());                    // .to_string() → String
    string(String::from("hi"));                   // String::from → String
    string("rust is fun!".to_owned());            // .to_owned() → String
    string("nice weather".into());                // .into() 推导为 String
    string(format!("Interpolation {}", "Station")); // format! → String
    string_slice(&String::from("abc")[0..1]);     // 字符串切片 → &str
    string_slice("  hello there ".trim());        // .trim() 返回 &str
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // .replace() → String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // .to_lowercase() → String
}

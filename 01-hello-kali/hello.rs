use std::string::String;

fn pouet(w: String) {
    println!("hello {}", w);
}

fn main() {
    pouet(String::from_str("kali"))
}

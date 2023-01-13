fn main() {
    let s = "1wexiev1wexiev-1wexiev-0".to_string();
    println!("{}", &s[s.find("-").unwrap() + 1..]);
}

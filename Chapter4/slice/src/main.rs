fn main() {
    let mut s = String::from("hello world");
    println!("{}", s);
    let r1 = &s[..5];
    println!("{}", r1);
    s.push_str(", japan");
    let r2 = &s[6..];
    println!("{}", r2);
    let r3 = &s[3..8];
    println!("{}", r3);
}


fn main() {
    let s = String::from("hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];
	// String经过&可以得到一个&str类型
    println!("{}", first_word(&s));
}

fn first_word(s: &str) -> &str {
    &s[..s.find(" ").unwrap_or(s.len())]
}
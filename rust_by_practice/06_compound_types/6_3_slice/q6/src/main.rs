// Fix errors
fn main() {
    let mut s = String::from("hello world");

    // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`. 
    let letter = first_letter(&s).to_string();

    s.clear(); // error!

    println!("the first letter is: {}", letter);
}
fn first_letter(s: &str) -> &str {
    &s[..1]
}
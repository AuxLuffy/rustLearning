fn main() {
    let mut s = String::from("hello world");
    let word = first_world(&s);
    println!("the first word is: {}", word);
    s.push_str("world");
}

fn first_world(s: &String) -> &str{
    for (i, &item) in s.as_bytes().iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}


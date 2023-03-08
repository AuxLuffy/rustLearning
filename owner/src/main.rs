fn main() {
    // let mut s = String::from("Hello ");
    // let len = caculate_len(&mut s);
    // println!("s = [ {} ], length of s is {}", s, len);
    // testString();
    // change(&mut s);
    // let s_ref = &mut s;
    // let len = caculate_len(s_ref);
    // println!(
    //     "after change, s = [ {} ], length of s is {}",
    //     s_ref,
    //     len
    // );
    // test_s_ref();
    test_val_var();
}

fn take_ownership(some_thing: &String) {
    // let len = some_thing.len();
    // println!("{}", len);

    // let mut x = 3;
    // let y = &x;
    // x = 4;
    // println!("x={}, y={}", x, y);
    // *y = 4;
    // let mut x = 3;
}

fn caculate_len(s: &mut String) -> usize {
    s.push_str("world!");
    s.len()
}

fn testString() {
    let s = "hello";
    let s1 = String::from(s);
    let s2 = s1.clone();
    let s3 = &s2;
    println!("s = {}, s1 = {}, s3 = {}", s, s1, s3)
}

fn change(some_thing: &mut String) {
    some_thing.push_str("abc")
}

fn test_s_ref() {
    let mut s = String::from("hello");
    let s_ref = &mut s;
    let len = caculate_len(s_ref);
    println!(
        "after change, s = [ {} ], length of s is {}",
        s_ref,
        len
    );


}



fn test_val_var(){
    let mut x = "5";
    let mut y = String::from("5");
    let mut z = &mut y;
    z.push_str("6");
    println!("x = {}, y = {}", x, z);
}
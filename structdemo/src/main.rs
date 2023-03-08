fn main() {
    let user1 = User {
        name: String::from("sunzf"),
        age: 18,
        id: String::from("123456"),
        email: String::from("sunzhangfei@zuoyebang.com"),
        active: true,
        sign_in_count: 1,
    };
    let mut user2 = User {
        email: String::from("liuquan@zuoyebang.com"),
        ..user1
    };
    user2.age = 10;
    println!(
        "user1.email = {}\nuser2.email = {}",
        user1.email, user2.email
    );

    let black = Color(0,0,0,0);
    black.0;
    black.1;
}

struct User {
    name: String,
    age: u32,
    id: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
/**
 * 元组结构体
 */
struct Color(i32, i32, i32, i32);

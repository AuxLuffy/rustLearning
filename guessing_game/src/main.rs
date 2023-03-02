use rand::Rng;
use std::{cmp::Ordering, io};
/**
 * 猜数游戏
 */
fn main() {
    println!("猜数！");
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("神秘数字是：{}", secret_number);
    loop {
        println!("猜测一个数: ");
        let mut _guess = String::new();
        io::stdin().read_line(&mut _guess).expect("无法读取行");
        println!("你猜测的数是：{}", _guess.trim());

        let _guess: u32 = match _guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Illegle input: {}", _guess);
                continue;
            }
        };
        match _guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!()
    }
}
